// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::visit::{Visit, visit_expr_call, visit_expr_macro, visit_stmt};

use crate::counting::hidden_dep_severity::HiddenDepSeverity;

const HIDDEN_DEP_TAILS: &[&str] = &[
    "Instant::now",
    "SystemTime::now",
    "random",
    "rand::random",
    "thread_rng",
    "rand::thread_rng",
    "fs::read",
    "fs::write",
    "File::open",
    "File::create",
    "env::var",
    "env::args",
    "process::exit",
    "process::abort",
    "abort",
    "thread::sleep",
];

#[derive(Default)]
pub struct HiddenDepsCounter {
    pub count: u32,
    pub weight: f64,
    pub labels: Vec<String>,
    severity: HiddenDepSeverity,
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
            if let Some(label) = Self::hidden_macro_label(&mac_name) {
                self.add_dep(label);
            }
        }
        visit_stmt(self, stmt);
    }

    fn visit_expr_call(&mut self, expr: &'ast syn::ExprCall) {
        if let Some(label) = Self::hidden_call_label(&expr.func) {
            self.add_dep(&label);
        }
        visit_expr_call(self, expr);
    }

    fn visit_expr_unsafe(&mut self, _expr: &'ast syn::ExprUnsafe) {
        self.add_dep("unsafe");
    }

    fn visit_expr_macro(&mut self, expr: &'ast syn::ExprMacro) {
        let mac_name = expr
            .mac
            .path
            .require_ident()
            .map(|i| i.to_string())
            .unwrap_or_default();
        if let Some(label) = Self::hidden_macro_label(&mac_name) {
            self.add_dep(label);
        }
        visit_expr_macro(self, expr);
    }
}

impl HiddenDepsCounter {
    fn add_dep(&mut self, label: &str) {
        self.count += 1;
        self.weight += self.severity.severity(label);
        self.labels.push(label.to_string());
    }

    fn hidden_call_label(expr: &syn::Expr) -> Option<String> {
        let syn::Expr::Path(path_expr) = expr else {
            return None;
        };
        let segments: Vec<String> = path_expr
            .path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect();
        Self::hidden_tail(&segments)
    }

    fn hidden_tail(segments: &[String]) -> Option<String> {
        if segments.is_empty() {
            return None;
        }
        let tail_start = segments.len().saturating_sub(2);
        let tail = segments[tail_start..].join("::");
        let qualified_ok = segments.len() <= 2 || segments[0] == "std" || segments[0] == "core";
        (qualified_ok && HIDDEN_DEP_TAILS.contains(&tail.as_str())).then_some(tail)
    }

    fn hidden_macro_label(mac: &str) -> Option<&str> {
        matches!(mac, "println" | "eprintln" | "print" | "eprint").then_some(mac)
    }
}
