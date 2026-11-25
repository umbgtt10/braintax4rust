// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::ToString;

use syn::Attribute;
use syn::ExprMacro;
use syn::Macro;
use syn::StmtMacro;

use syn::visit::{Visit, visit_attribute, visit_expr_macro, visit_stmt_macro};

use crate::counting::derive_attr_scorer::DeriveAttrScorer;

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

const KNOWN_ATTRIBUTES: &[&str] = &[
    "allow",
    "automatically_derived",
    "cfg",
    "cfg_attr",
    "cold",
    "collapse_debuginfo",
    "crate_name",
    "crate_type",
    "debugger_visualizer",
    "deny",
    "deprecated",
    "derive",
    "doc",
    "expect",
    "export_name",
    "feature",
    "forbid",
    "global_allocator",
    "ignore",
    "inline",
    "instruction_set",
    "link",
    "link_name",
    "link_ordinal",
    "link_section",
    "macro_export",
    "macro_use",
    "must_use",
    "naked",
    "no_builtins",
    "no_implicit_prelude",
    "no_link",
    "no_main",
    "no_mangle",
    "no_std",
    "non_exhaustive",
    "panic_handler",
    "path",
    "proc_macro",
    "proc_macro_attribute",
    "proc_macro_derive",
    "recursion_limit",
    "repr",
    "should_panic",
    "target_feature",
    "test",
    "track_caller",
    "type_length_limit",
    "used",
    "warn",
    "windows_subsystem",
];

#[derive(Default)]
pub struct MacroCounter {
    count: u32,
}

impl MacroCounter {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    fn is_known_macro(name: &str) -> bool {
        KNOWN_STD_MACROS.contains(&name)
    }

    fn macro_cost(mac: &Macro) -> u32 {
        let name = mac
            .path
            .require_ident()
            .map(|i| i.to_string())
            .unwrap_or_default();
        if Self::is_known_macro(&name) { 0 } else { 3 }
    }
}

impl<'ast> Visit<'ast> for MacroCounter {
    fn visit_expr_macro(&mut self, expr: &'ast ExprMacro) {
        self.count += Self::macro_cost(&expr.mac);
        visit_expr_macro(self, expr);
    }

    fn visit_stmt_macro(&mut self, stmt: &'ast StmtMacro) {
        self.count += Self::macro_cost(&stmt.mac);
        visit_stmt_macro(self, stmt);
    }

    fn visit_attribute(&mut self, attr: &'ast Attribute) {
        if let Some(ident) = attr.path().get_ident() {
            let name = ident.to_string();
            if name == "derive" {
                Self::score_derive_attr(self, attr);
            } else if !Self::is_known_attr(&name) {
                self.count += 3;
            }
        }
        visit_attribute(self, attr);
    }
}

impl MacroCounter {
    fn is_known_attr(name: &str) -> bool {
        KNOWN_ATTRIBUTES.contains(&name)
    }

    fn score_derive_attr(&mut self, attr: &Attribute) {
        self.count += DeriveAttrScorer::new().score(attr);
    }
}
