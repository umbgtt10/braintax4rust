// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::visit::Visit;

#[derive(Default)]
pub struct NameOpacityCounter {
    pub score: u32,
}

impl NameOpacityCounter {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn score_ident(name: &str) -> u32 {
        match name.len() {
            1 => 2,
            2 | 3 => 1,
            _ => 0,
        }
    }

    pub fn visit_params(
        &mut self,
        inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::Token![,]>,
    ) {
        for input in inputs {
            if let syn::FnArg::Typed(pat_type) = input
                && let syn::Pat::Ident(pat_ident) = pat_type.pat.as_ref()
            {
                self.score += Self::score_ident(&pat_ident.ident.to_string());
            }
        }
    }
}

impl<'ast> Visit<'ast> for NameOpacityCounter {
    fn visit_pat(&mut self, pat: &'ast syn::Pat) {
        if let syn::Pat::Ident(pat_ident) = pat {
            self.score += Self::score_ident(&pat_ident.ident.to_string());
        }
        syn::visit::visit_pat(self, pat);
    }
}
