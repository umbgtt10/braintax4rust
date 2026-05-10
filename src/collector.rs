// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;

use quote::ToTokens;
use syn::Attribute;
use syn::visit::Visit;

use crate::complexity_visitor::ComplexityVisitor;
use crate::function_complexity::FunctionComplexity;

#[derive(Debug)]
pub struct Collector {
    functions: Vec<FunctionComplexity>,
    current_file: String,
    current_module: String,
}

impl Collector {
    fn new(file: String, module: String) -> Self {
        Self {
            functions: Vec::new(),
            current_file: file,
            current_module: module,
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
            _ => {}
        }
    }
}

impl Collector {
    fn visit_fn(&mut self, item_fn: &syn::ItemFn) {
        let name = item_fn.sig.ident.to_string();
        let mut visitor = ComplexityVisitor::new();
        visitor.visit_block(&item_fn.block);

        self.functions.push(FunctionComplexity {
            name,
            file: self.current_file.clone(),
            module: self.current_module.clone(),
            cyclomatic: visitor.complexity,
        });
    }

    fn visit_mod(&mut self, item_mod: &syn::ItemMod) {
        if let Some((_, items)) = &item_mod.content {
            for inner in items {
                self.visit_item(inner);
            }
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
                    .map(|list| list.to_token_stream().to_string().contains("test"))
                    .unwrap_or(false),
                _ => false,
            }
        })
    }
}
