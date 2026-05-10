// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::visit::Visit;

use crate::derive_attr_scorer::DeriveAttrScorer;

#[derive(Default)]
pub struct MacroCounter {
    pub count: u32,
}

impl MacroCounter {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn is_known_macro(name: &str) -> bool {
        KNOWN_STD_MACROS.contains(&name)
    }
}

const KNOWN_STD_MACROS: &[&str] = &[
    "println",
    "eprintln",
    "print",
    "eprint",
    "format",
    "write",
    "writeln",
    "vec",
    "assert",
    "assert_eq",
    "assert_ne",
    "panic",
    "unreachable",
    "unimplemented",
    "todo",
    "include",
    "include_str",
    "include_bytes",
    "concat",
    "stringify",
    "matches",
    "cfg",
    "file",
    "line",
    "column",
    "compile_error",
    "env",
    "option_env",
];

impl<'ast> Visit<'ast> for MacroCounter {
    fn visit_expr_macro(&mut self, expr: &'ast syn::ExprMacro) {
        let name = expr
            .mac
            .path
            .require_ident()
            .map(|i| i.to_string())
            .unwrap_or_default();
        if !Self::is_known_macro(&name) {
            self.count += 3;
        }
        syn::visit::visit_expr_macro(self, expr);
    }

    fn visit_stmt_macro(&mut self, stmt: &'ast syn::StmtMacro) {
        let name = stmt
            .mac
            .path
            .require_ident()
            .map(|i| i.to_string())
            .unwrap_or_default();
        if !Self::is_known_macro(&name) {
            self.count += 3;
        }
        syn::visit::visit_stmt_macro(self, stmt);
    }

    fn visit_attribute(&mut self, attr: &'ast syn::Attribute) {
        if let Some(ident) = attr.path().get_ident() {
            let name = ident.to_string();
            if name == "derive" {
                Self::score_derive_attr(self, attr);
            } else if !Self::is_known_attr(&name) {
                self.count += 3;
            }
        }
        syn::visit::visit_attribute(self, attr);
    }
}

impl MacroCounter {
    fn score_derive_attr(&mut self, attr: &syn::Attribute) {
        self.count += DeriveAttrScorer::new().score(attr);
    }

    fn is_known_attr(name: &str) -> bool {
        Self::is_known_macro(name)
            || name == "cfg"
            || name == "cfg_attr"
            || name == "allow"
            || name == "deny"
            || name == "warn"
    }
}
