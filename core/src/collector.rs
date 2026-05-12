// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::path::Path;

use syn::Attribute;
use syn::visit::Visit;

use crate::complexity_visitor::ComplexityVisitor;
use crate::function_complexity::FunctionComplexity;
use crate::generics_counter::GenericsCounter;
use crate::hidden_deps_counter::HiddenDepsCounter;
use crate::macro_counter::MacroCounter;
use crate::name_opacity_counter::NameOpacityCounter;

#[derive(Debug, Clone)]
pub struct TraitInfo {
    pub methods: u32,
    pub assoc_types: u32,
    pub supertraits: u32,
}

#[derive(Debug)]
pub struct Collector {
    functions: Vec<FunctionComplexity>,
    current_file: String,
    current_module: String,
    current_depth: u32,
    traits: HashMap<String, TraitInfo>,
}

impl Collector {
    fn new(file: String, module: String) -> Self {
        let depth = Self::module_depth(&module);
        Self {
            functions: Vec::new(),
            current_file: file,
            current_module: module,
            current_depth: depth,
            traits: HashMap::new(),
        }
    }

    pub fn collect(source: &str, path: &Path, root: &Path) -> Vec<FunctionComplexity> {
        let file = path.to_string_lossy().replace('\\', "/");
        let module = Self::module_from_path(path, root);
        let syntax = match syn::parse_file(source) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        let mut collector = Self::new(file, module);
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

    fn trait_info(trait_item: &syn::ItemTrait) -> TraitInfo {
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
        let supertraits = trait_item.supertraits.len() as u32;
        TraitInfo {
            methods,
            assoc_types,
            supertraits,
        }
    }

    fn compute_trait_factor(name: &str, info: &TraitInfo) -> f64 {
        let known = [
            "Debug",
            "Clone",
            "Copy",
            "Default",
            "Eq",
            "PartialEq",
            "Ord",
            "PartialOrd",
            "Hash",
            "Display",
            "Iterator",
            "Into",
            "From",
            "Send",
            "Sync",
            "Drop",
        ];
        if known.contains(&name) {
            return 0.80;
        }
        let method_penalty = (info.methods.saturating_sub(3)) as f64 * 0.02;
        let base = if info.assoc_types > 0 || info.supertraits > 0 {
            1.15 + method_penalty
        } else {
            0.90 + method_penalty
        };
        let assoc_penalty = if info.assoc_types > 0 { 0.10 } else { 0.0 };
        let super_penalty = if info.supertraits > 0 { 0.10 } else { 0.0 };
        base + assoc_penalty + super_penalty
    }
}

impl<'ast> Visit<'ast> for Collector {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        match item {
            syn::Item::Fn(item_fn) if !Self::has_test_attr(&item_fn.attrs) => {
                self.visit_fn(item_fn);
            }
            syn::Item::Mod(item_mod) if !Self::has_test_attr(&item_mod.attrs) => {
                self.visit_mod(item_mod);
            }
            syn::Item::Trait(trait_item) => {
                let name = trait_item.ident.to_string();
                let info = Self::trait_info(trait_item);
                self.traits.insert(name, info);
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
                    });
                    Self::compute_trait_factor(&name, &info)
                } else {
                    1.0
                };
                for inner in &item_impl.items {
                    self.visit_impl_item(inner, trait_factor);
                }
            }
            _ => {}
        }
    }
}

impl Collector {
    fn push_fn(
        &mut self,
        name: String,
        block: &syn::Block,
        attrs: &[syn::Attribute],
        trait_factor: f64,
        param_opacity: u32,
        generics: u32,
    ) {
        let mut visitor = ComplexityVisitor::new();
        visitor.visit_block(block);
        let mut hidden = HiddenDepsCounter::new();
        hidden.visit_block(block);
        let mut names = NameOpacityCounter::new();
        names.visit_block(block);
        let name_opacity = param_opacity + names.score;
        let mut macros = MacroCounter::new();
        macros.visit_block(block);
        let cfg_gates = Self::count_cfg_gates(attrs);
        let depth = self.current_depth;
        let components = crate::default_scorer::BraintaxComponents {
            cfg_gates,
            cyclomatic: visitor.complexity,
            hidden_deps: hidden.count,
            depth,
            trait_factor,
            name_opacity,
            macro_density: macros.count,
            generics,
        };
        self.functions.push(FunctionComplexity {
            name,
            file: self.current_file.clone(),
            module: self.current_module.clone(),
            cyclomatic: visitor.complexity,
            cfg_gates,
            hidden_deps: hidden.count,
            depth,
            trait_factor,
            braintax: crate::default_scorer::compute_braintax(&components),
        });
    }

    fn count_cfg_gates(attrs: &[syn::Attribute]) -> u32 {
        attrs
            .iter()
            .filter(|attr| {
                let ident = attr.path().get_ident().map(|i| i.to_string());
                matches!(ident.as_deref(), Some("cfg") | Some("cfg_attr"))
            })
            .count() as u32
    }

    fn visit_fn(&mut self, item_fn: &syn::ItemFn) {
        let mut names = NameOpacityCounter::new();
        names.visit_params(&item_fn.sig.inputs);
        let generics = GenericsCounter::score_generics(
            &item_fn.sig.generics.params,
            &item_fn.sig.generics.where_clause,
        );
        self.push_fn(
            item_fn.sig.ident.to_string(),
            &item_fn.block,
            &item_fn.attrs,
            1.0,
            names.score,
            generics,
        );
    }

    fn visit_mod(&mut self, item_mod: &syn::ItemMod) {
        if let Some((_, items)) = &item_mod.content {
            for inner in items {
                self.visit_item(inner);
            }
        }
    }

    fn visit_impl_item(&mut self, item: &syn::ImplItem, trait_factor: f64) {
        if let syn::ImplItem::Fn(item_fn) = item
            && !Self::has_test_attr(&item_fn.attrs)
        {
            let mut names = NameOpacityCounter::new();
            names.visit_params(&item_fn.sig.inputs);
            let generics = GenericsCounter::score_generics(
                &item_fn.sig.generics.params,
                &item_fn.sig.generics.where_clause,
            );
            self.push_fn(
                item_fn.sig.ident.to_string(),
                &item_fn.block,
                &item_fn.attrs,
                trait_factor,
                names.score,
                generics,
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
