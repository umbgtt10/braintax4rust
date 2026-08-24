// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT
use syn::Attribute;
use syn::ExprBinary;
use syn::ExprBreak;
use syn::ExprContinue;
use syn::ExprForLoop;
use syn::ExprIf;
use syn::ExprLoop;
use syn::ExprMatch;
use syn::ExprReturn;
use syn::ExprTry;
use syn::ExprWhile;

use syn::visit::{
    Visit, visit_expr_binary, visit_expr_for_loop, visit_expr_loop, visit_expr_match,
    visit_expr_while,
};

pub struct ComplexityVisitor {
    pub complexity: u32,
    pub cfg_body_gates: u32,
}

impl ComplexityVisitor {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ComplexityVisitor {
    fn default() -> Self {
        Self {
            complexity: 1,
            cfg_body_gates: 0,
        }
    }
}

impl<'ast> Visit<'ast> for ComplexityVisitor {
    fn visit_attribute(&mut self, attr: &'ast Attribute) {
        let ident = attr.path().get_ident().map(|i| i.to_string());
        if matches!(ident.as_deref(), Some("cfg") | Some("cfg_attr")) {
            self.cfg_body_gates += 1;
        }
    }

    fn visit_expr_if(&mut self, expr: &'ast ExprIf) {
        self.complexity += 1;
        self.visit_expr(&expr.cond);
        self.visit_block(&expr.then_branch);
        if let Some((_, else_branch)) = &expr.else_branch {
            self.visit_expr(else_branch);
        }
    }

    fn visit_expr_while(&mut self, expr: &'ast ExprWhile) {
        self.complexity += 1;
        visit_expr_while(self, expr);
    }

    fn visit_expr_for_loop(&mut self, expr: &'ast ExprForLoop) {
        self.complexity += 1;
        visit_expr_for_loop(self, expr);
    }

    fn visit_expr_loop(&mut self, expr: &'ast ExprLoop) {
        self.complexity += 1;
        visit_expr_loop(self, expr);
    }

    fn visit_expr_match(&mut self, expr: &'ast ExprMatch) {
        self.complexity += 1;
        let arm_count = expr.arms.len();
        if arm_count > 1 {
            self.complexity += (arm_count - 1) as u32;
        }
        for arm in &expr.arms {
            if arm.guard.is_some() {
                self.complexity += 1;
            }
        }
        visit_expr_match(self, expr);
    }

    fn visit_expr_binary(&mut self, expr: &'ast ExprBinary) {
        if matches!(expr.op, syn::BinOp::And(_) | syn::BinOp::Or(_)) {
            self.complexity += 1;
        }
        visit_expr_binary(self, expr);
    }

    fn visit_expr_try(&mut self, _expr: &'ast ExprTry) {
        self.complexity += 1;
    }

    fn visit_expr_return(&mut self, _expr: &'ast ExprReturn) {
        self.complexity += 1;
    }

    fn visit_expr_break(&mut self, _expr: &'ast ExprBreak) {
        self.complexity += 1;
    }

    fn visit_expr_continue(&mut self, _expr: &'ast ExprContinue) {
        self.complexity += 1;
    }
}
