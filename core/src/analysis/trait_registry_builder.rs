// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::path::PathBuf;

use crate::analysis::collector::Collector;
use crate::analysis::trait_info::TraitInfo;
use syn::parse_file;

pub struct TraitRegistryBuilder {
    traits: HashMap<String, TraitInfo>,
}

impl TraitRegistryBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            traits: HashMap::new(),
        }
    }

    #[must_use]
    pub fn build(mut self, files: &[(PathBuf, String)]) -> HashMap<String, TraitInfo> {
        for (_, source) in files {
            self.scan_trait_definitions(source);
        }
        for (_, source) in files {
            self.scan_trait_impls(source);
        }
        self.traits
    }

    fn scan_trait_definitions(&mut self, source: &str) {
        let Ok(syntax) = parse_file(source) else {
            return;
        };
        for item in &syntax.items {
            if let syn::Item::Trait(trait_item) = item {
                let name = trait_item.ident.to_string();
                self.traits.insert(name, Collector::trait_info(trait_item));
            }
        }
    }

    fn scan_trait_impls(&mut self, source: &str) {
        let Ok(syntax) = parse_file(source) else {
            return;
        };
        for item in &syntax.items {
            self.record_impl(item);
        }
    }

    fn record_impl(&mut self, item: &syn::Item) {
        let syn::Item::Impl(item_impl) = item else {
            return;
        };
        let Some((_, path, _)) = &item_impl.trait_ else {
            return;
        };
        let name = path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default();
        if let Some(info) = self.traits.get_mut(&name) {
            info.impl_count += 1;
        }
    }
}

impl Default for TraitRegistryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
