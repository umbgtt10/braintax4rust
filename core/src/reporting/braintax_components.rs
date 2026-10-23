// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, Default)]
pub struct BraintaxComponents {
    cfg_gates: u32,
    cyclomatic: u32,
    hidden_dep_weight: f64,
    depth: u32,
    trait_factor: f64,
    name_opacity: u32,
    macro_density: u32,
    generics: u32,
    self_ref_cost: f64,
    return_complexity: f64,
}

impl BraintaxComponents {
    pub fn cfg_gates(&self) -> u32 {
        self.cfg_gates
    }

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

    pub fn cyclomatic(&self) -> u32 {
        self.cyclomatic
    }

    pub fn depth(&self) -> u32 {
        self.depth
    }

    pub fn generics(&self) -> u32 {
        self.generics
    }

    pub fn hidden_dep_weight(&self) -> f64 {
        self.hidden_dep_weight
    }

    pub fn macro_density(&self) -> u32 {
        self.macro_density
    }

    pub fn name_opacity(&self) -> u32 {
        self.name_opacity
    }

    pub fn return_complexity(&self) -> f64 {
        self.return_complexity
    }

    pub fn self_ref_cost(&self) -> f64 {
        self.self_ref_cost
    }

    pub fn trait_factor(&self) -> f64 {
        self.trait_factor
    }

    #[must_use]
    pub fn with_cfg_gates(self, cfg_gates: u32) -> Self {
        Self { cfg_gates, ..self }
    }

    #[must_use]
    pub fn with_cyclomatic(self, cyclomatic: u32) -> Self {
        Self { cyclomatic, ..self }
    }

    #[must_use]
    pub fn with_depth(self, depth: u32) -> Self {
        Self { depth, ..self }
    }

    #[must_use]
    pub fn with_generics(self, generics: u32) -> Self {
        Self { generics, ..self }
    }

    #[must_use]
    pub fn with_hidden_dep_weight(self, hidden_dep_weight: f64) -> Self {
        Self {
            hidden_dep_weight,
            ..self
        }
    }

    #[must_use]
    pub fn with_macro_density(self, macro_density: u32) -> Self {
        Self {
            macro_density,
            ..self
        }
    }

    #[must_use]
    pub fn with_name_opacity(self, name_opacity: u32) -> Self {
        Self {
            name_opacity,
            ..self
        }
    }

    #[must_use]
    pub fn with_return_complexity(self, return_complexity: f64) -> Self {
        Self {
            return_complexity,
            ..self
        }
    }

    #[must_use]
    pub fn with_self_ref_cost(self, self_ref_cost: f64) -> Self {
        Self {
            self_ref_cost,
            ..self
        }
    }

    #[must_use]
    pub fn with_trait_factor(self, trait_factor: f64) -> Self {
        Self {
            trait_factor,
            ..self
        }
    }
}
