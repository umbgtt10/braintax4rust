// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::visit::Visit;

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

    fn is_known_derive(name: &str) -> bool {
        KNOWN_DERIVES.contains(&name)
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

const KNOWN_DERIVES: &[&str] = &[
    "Debug",
    "Clone",
    "Copy",
    "Default",
    "Eq",
    "PartialEq",
    "Ord",
    "PartialOrd",
    "Hash",
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
                if let Ok(meta_list) = attr.meta.require_list() {
                    let s = format!("{}", meta_list.tokens);
                    for derive_name in s.trim_start_matches('(').trim_end_matches(')').split(',') {
                        let cleaned = derive_name.trim();
                        if !cleaned.is_empty() && !Self::is_known_derive(cleaned) {
                            self.count += 3;
                        }
                    }
                }
            } else if !Self::is_known_macro(&name)
                && name != "cfg"
                && name != "cfg_attr"
                && name != "allow"
                && name != "deny"
                && name != "warn"
            {
                self.count += 3;
            }
        }
        syn::visit::visit_attribute(self, attr);
    }
}
