// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::visit::Visit;

#[derive(Default)]
pub struct HiddenDepsCounter {
    pub count: u32,
}

impl HiddenDepsCounter {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'ast> Visit<'ast> for HiddenDepsCounter {
    fn visit_stmt(&mut self, stmt: &'ast syn::Stmt) {
        if let syn::Stmt::Macro(stmt_macro) = stmt {
            let mac_name = stmt_macro
                .mac
                .path
                .require_ident()
                .map(|i| i.to_string())
                .unwrap_or_default();
            if Self::is_hidden_macro(&mac_name) {
                self.count += 1;
            }
        }
        syn::visit::visit_stmt(self, stmt);
    }

    fn visit_expr_call(&mut self, expr: &'ast syn::ExprCall) {
        if Self::is_hidden_call(&expr.func) {
            self.count += 1;
        }
        syn::visit::visit_expr_call(self, expr);
    }

    fn visit_expr_unsafe(&mut self, _expr: &'ast syn::ExprUnsafe) {
        self.count += 1;
    }

    fn visit_expr_macro(&mut self, expr: &'ast syn::ExprMacro) {
        let mac_name = expr
            .mac
            .path
            .require_ident()
            .map(|i| i.to_string())
            .unwrap_or_default();
        if Self::is_hidden_macro(&mac_name) {
            self.count += 1;
        }
        syn::visit::visit_expr_macro(self, expr);
    }
}

impl HiddenDepsCounter {
    fn is_hidden_call(expr: &syn::Expr) -> bool {
        let syn::Expr::Path(path_expr) = expr else {
            return false;
        };
        let segments: Vec<String> = path_expr
            .path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect();
        let path_str = segments.join("::");
        Self::is_hidden_path(&path_str)
    }

    fn is_hidden_path(path: &str) -> bool {
        matches!(
            path,
            "Instant::now"
                | "SystemTime::now"
                | "random"
                | "thread_rng"
                | "std::fs::read"
                | "std::fs::write"
                | "File::open"
                | "File::create"
                | "env::var"
                | "std::env::var"
                | "env::args"
                | "std::env::args"
                | "process::exit"
                | "std::process::exit"
                | "abort"
                | "thread::sleep"
                | "std::thread::sleep"
        )
    }

    fn is_hidden_macro(mac: &str) -> bool {
        matches!(mac, "println" | "eprintln" | "print" | "eprint")
    }
}
