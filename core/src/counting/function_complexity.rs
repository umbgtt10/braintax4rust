// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FunctionComplexity {
    name: String,
    file: String,
    module: String,
    cyclomatic: u32,
    cfg_gates: u32,
    hidden_deps: u32,
    hidden_dep_weight: f64,
    hidden_dep_labels: Vec<String>,
    depth: u32,
    trait_factor: f64,
    braintax: f64,
    braintax_normalized: u32,
}

impl FunctionComplexity {
    pub fn braintax(&self) -> f64 {
        self.braintax
    }

    pub fn braintax_normalized(&self) -> u32 {
        self.braintax_normalized
    }

    pub fn cfg_gates(&self) -> u32 {
        self.cfg_gates
    }

    pub fn cyclomatic(&self) -> u32 {
        self.cyclomatic
    }

    pub fn depth(&self) -> u32 {
        self.depth
    }

    pub fn file(&self) -> &str {
        &self.file
    }

    pub fn hidden_dep_labels(&self) -> &[String] {
        &self.hidden_dep_labels
    }

    pub fn hidden_dep_weight(&self) -> f64 {
        self.hidden_dep_weight
    }

    pub fn hidden_deps(&self) -> u32 {
        self.hidden_deps
    }

    pub fn module(&self) -> &str {
        &self.module
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn trait_factor(&self) -> f64 {
        self.trait_factor
    }

    #[must_use]
    pub fn with_braintax(self, braintax: f64) -> Self {
        Self { braintax, ..self }
    }

    #[must_use]
    pub fn with_braintax_normalized(self, braintax_normalized: u32) -> Self {
        Self {
            braintax_normalized,
            ..self
        }
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
    pub fn with_file(self, file: String) -> Self {
        Self { file, ..self }
    }

    #[must_use]
    pub fn with_hidden_dep_labels(self, hidden_dep_labels: Vec<String>) -> Self {
        Self {
            hidden_dep_labels,
            ..self
        }
    }

    #[must_use]
    pub fn with_hidden_dep_weight(self, hidden_dep_weight: f64) -> Self {
        Self {
            hidden_dep_weight,
            ..self
        }
    }

    #[must_use]
    pub fn with_hidden_deps(self, hidden_deps: u32) -> Self {
        Self {
            hidden_deps,
            ..self
        }
    }

    #[must_use]
    pub fn with_module(self, module: String) -> Self {
        Self { module, ..self }
    }

    #[must_use]
    pub fn with_name(self, name: String) -> Self {
        Self { name, ..self }
    }

    #[must_use]
    pub fn with_trait_factor(self, trait_factor: f64) -> Self {
        Self {
            trait_factor,
            ..self
        }
    }
}
