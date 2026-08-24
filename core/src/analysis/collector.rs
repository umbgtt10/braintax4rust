// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::analysis::trait_info::TraitInfo;
use crate::counting::signature_scorer::SignatureScorer;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use syn::Attribute;
use syn::Block;
use syn::ImplItem;
use syn::Item;
use syn::ItemFn;
use syn::ItemMod;
use syn::ItemTrait;
use syn::visit::Visit;

use crate::analysis::trait_registry_builder::TraitRegistryBuilder;
use crate::counting::complexity_visitor::ComplexityVisitor;
use crate::counting::function_complexity::FunctionComplexity;
use crate::counting::generics_counter::GenericsCounter;
use crate::counting::hidden_deps_counter::HiddenDepsCounter;
use crate::counting::macro_counter::MacroCounter;
use crate::counting::name_opacity_counter::NameOpacityCounter;
use crate::reporting::braintax_normalizer::BraintaxNormalizer;
use crate::reporting::default_scorer::{BraintaxComponents, compute_braintax};
use syn::parse_file;

#[derive(Debug)]
pub struct Collector<'a> {
    functions: Vec<FunctionComplexity>,
    current_file: String,
    current_module: String,
    current_depth: u32,
    traits: &'a HashMap<String, TraitInfo>,
    normalizer: BraintaxNormalizer,
}

impl<'a> Collector<'a> {
    fn new(file: String, module: String, traits: &'a HashMap<String, TraitInfo>) -> Self {
        let depth = Self::module_depth(&module);
        Self {
            functions: Vec::new(),
            current_file: file,
            current_module: module,
            current_depth: depth,
            traits,
            normalizer: BraintaxNormalizer::new(),
        }
    }

    pub fn build_trait_registry(files: &[(PathBuf, String)]) -> HashMap<String, TraitInfo> {
        TraitRegistryBuilder::new().build(files)
    }

    pub fn collect(
        source: &str,
        path: &Path,
        root: &Path,
        traits: &'a HashMap<String, TraitInfo>,
    ) -> Vec<FunctionComplexity> {
        let file = path.to_string_lossy().replace('\\', "/");
        let module = Self::module_from_path(path, root);
        let syntax = match parse_file(source) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        let mut collector = Self::new(file, module, traits);

        for item in &syntax.items {
            collector.visit_item(item);
        }
        collector.functions
    }

    fn module_depth(module: &str) -> u32 {
        if module == "." {
            1
        } else {
            module.split('/').count() as u32 + 1
        }
    }

    fn module_from_path(path: &Path, root: &Path) -> String {
        let relative = path.strip_prefix(root).unwrap_or(path);
        let s = relative.to_string_lossy().replace('\\', "/");
        let without_src = s.strip_prefix("src/").map(|s| s.to_string()).unwrap_or(s);
        if let Some(pos) = without_src.rfind('/') {
            without_src[..pos].to_string()
        } else {
            ".".to_string()
        }
    }

    pub(crate) fn trait_info(trait_item: &ItemTrait) -> TraitInfo {
        let methods = trait_item
            .items
            .iter()
            .filter(|i| matches!(i, syn::TraitItem::Fn(_)))
            .count() as u32;
        let assoc_types = trait_item
            .items
            .iter()
            .filter(|i| matches!(i, syn::TraitItem::Type(_)))
            .count() as u32;
        let supertraits = trait_item
            .supertraits
            .iter()
            .filter(|bound| !SignatureScorer::is_marker_supertrait(bound))
            .count() as u32;
        TraitInfo {
            methods,
            assoc_types,
            supertraits,
            impl_count: 0,
        }
    }
}

impl<'ast, 'a> Visit<'ast> for Collector<'a> {
    fn visit_item(&mut self, item: &'ast Item) {
        match item {
            syn::Item::Fn(item_fn) if !Self::has_test_attr(&item_fn.attrs) => {
                self.visit_fn(item_fn);
            }
            syn::Item::Mod(item_mod) if !Self::has_test_attr(&item_mod.attrs) => {
                self.visit_mod(item_mod);
            }
            syn::Item::Impl(item_impl) => {
                let trait_factor = if let Some((_, path, _)) = &item_impl.trait_ {
                    let name = path
                        .segments
                        .last()
                        .map(|s| s.ident.to_string())
                        .unwrap_or_default();
                    let info = self.traits.get(&name).cloned().unwrap_or(TraitInfo {
                        methods: 0,
                        assoc_types: 0,
                        supertraits: 0,
                        impl_count: 0,
                    });
                    SignatureScorer::trait_factor(&name, &info)
                } else {
                    0.95
                };
                for inner in &item_impl.items {
                    self.visit_impl_item(inner, trait_factor);
                }
            }
            _ => {}
        }
    }
}

struct FnInput {
    trait_factor: f64,
    param_opacity: u32,
    generics: u32,
    self_ref_cost: f64,
    return_complexity: f64,
}

impl<'a> Collector<'a> {
    fn push_fn(&mut self, name: String, block: &Block, attrs: &[Attribute], input: FnInput) {
        let mut visitor = ComplexityVisitor::new();
        visitor.visit_block(block);
        let mut hidden = HiddenDepsCounter::new();
        hidden.visit_block(block);
        let mut names = NameOpacityCounter::new();
        names.visit_block(block);
        let name_opacity = input.param_opacity + names.score;
        let mut macros = MacroCounter::new();
        macros.visit_block(block);
        let cfg_gates = Self::count_cfg_gates(attrs);
        let depth = self.current_depth;
        let components = BraintaxComponents {
            cfg_gates,
            cyclomatic: visitor.complexity,
            hidden_dep_weight: hidden.weight,
            depth,
            trait_factor: input.trait_factor,
            name_opacity,
            macro_density: macros.count,
            generics: input.generics,
            self_ref_cost: input.self_ref_cost,
            return_complexity: input.return_complexity,
        };
        let braintax = compute_braintax(&components);
        self.functions.push(FunctionComplexity {
            name,
            file: self.current_file.clone(),
            module: self.current_module.clone(),
            cyclomatic: visitor.complexity,
            cfg_gates,
            hidden_deps: hidden.count,
            hidden_dep_weight: hidden.weight,
            hidden_dep_labels: hidden.labels,
            depth,
            trait_factor: input.trait_factor,
            braintax,
            braintax_normalized: self.normalizer.normalize(braintax),
        });
    }

    fn count_cfg_gates(attrs: &[Attribute]) -> u32 {
        attrs
            .iter()
            .filter(|attr| {
                let ident = attr.path().get_ident().map(|i| i.to_string());
                matches!(ident.as_deref(), Some("cfg") | Some("cfg_attr"))
            })
            .count() as u32
    }

    fn visit_fn(&mut self, item_fn: &ItemFn) {
        let mut names = NameOpacityCounter::new();
        names.visit_params(&item_fn.sig.inputs);
        let generics = GenericsCounter::score_generics(
            &item_fn.sig.generics.params,
            &item_fn.sig.generics.where_clause,
        );
        let ret_complexity = SignatureScorer::return_complexity(&item_fn.sig.output);
        self.push_fn(
            item_fn.sig.ident.to_string(),
            &item_fn.block,
            &item_fn.attrs,
            FnInput {
                trait_factor: 1.0,
                param_opacity: names.score,
                generics,
                self_ref_cost: 0.0,
                return_complexity: ret_complexity,
            },
        );
    }

    fn visit_mod(&mut self, item_mod: &ItemMod) {
        if let Some((_, items)) = &item_mod.content {
            for inner in items {
                self.visit_item(inner);
            }
        }
    }

    fn visit_impl_item(&mut self, item: &ImplItem, trait_factor: f64) {
        if let syn::ImplItem::Fn(item_fn) = item
            && !Self::has_test_attr(&item_fn.attrs)
        {
            let mut names = NameOpacityCounter::new();
            names.visit_params(&item_fn.sig.inputs);
            let generics = GenericsCounter::score_generics(
                &item_fn.sig.generics.params,
                &item_fn.sig.generics.where_clause,
            );
            let self_cost = SignatureScorer::self_ref_cost(&item_fn.sig.inputs);
            let ret_complexity = SignatureScorer::return_complexity(&item_fn.sig.output);
            self.push_fn(
                item_fn.sig.ident.to_string(),
                &item_fn.block,
                &item_fn.attrs,
                FnInput {
                    trait_factor,
                    param_opacity: names.score,
                    generics,
                    self_ref_cost: self_cost,
                    return_complexity: ret_complexity,
                },
            );
        }
    }

    fn has_test_attr(attrs: &[Attribute]) -> bool {
        attrs.iter().any(|attr| {
            let ident = attr.path().get_ident().map(|i| i.to_string());
            match ident.as_deref() {
                Some("test") => true,
                Some("cfg") => attr
                    .meta
                    .require_list()
                    .ok()
                    .map(|list| format!("{}", list.tokens).contains("test"))
                    .unwrap_or(false),
                _ => false,
            }
        })
    }
}
