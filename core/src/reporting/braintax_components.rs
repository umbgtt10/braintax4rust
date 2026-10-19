// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone)]
pub struct BraintaxComponents {
    pub cfg_gates: u32,
    pub cyclomatic: u32,
    pub hidden_dep_weight: f64,
    pub depth: u32,
    pub trait_factor: f64,
    pub name_opacity: u32,
    pub macro_density: u32,
    pub generics: u32,
    pub self_ref_cost: f64,
    pub return_complexity: f64,
}

impl BraintaxComponents {
    pub fn compute_braintax(&self) -> f64 {
        let cfg_factor = if self.cfg_gates > 0 {
            2.0f64.powi(self.cfg_gates as i32)
        } else {
            1.0
        };
        let depth_factor = 1.0 + (self.depth.saturating_sub(1) as f64) * 0.15;
        self.cyclomatic as f64 * cfg_factor * depth_factor * self.trait_factor
            + self.hidden_dep_weight
            + self.name_opacity as f64
            + self.macro_density as f64
            + self.generics as f64
            + self.self_ref_cost
            + self.return_complexity
    }
}
