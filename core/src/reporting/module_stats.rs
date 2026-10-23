// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModuleStats {
    path: String,
    function_count: usize,
    avg_cyclomatic: f64,
    max_cyclomatic: u32,
    total_cyclomatic: u32,
    avg_braintax: f64,
    max_braintax: f64,
    total_braintax: f64,
    braintax_normalized: u32,
}

impl ModuleStats {
    pub fn avg_braintax(&self) -> f64 {
        self.avg_braintax
    }

    pub fn avg_cyclomatic(&self) -> f64 {
        self.avg_cyclomatic
    }

    pub fn braintax_normalized(&self) -> u32 {
        self.braintax_normalized
    }

    pub fn function_count(&self) -> usize {
        self.function_count
    }

    pub fn max_braintax(&self) -> f64 {
        self.max_braintax
    }

    pub fn max_cyclomatic(&self) -> u32 {
        self.max_cyclomatic
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn total_braintax(&self) -> f64 {
        self.total_braintax
    }

    pub fn total_cyclomatic(&self) -> u32 {
        self.total_cyclomatic
    }

    #[must_use]
    pub fn with_avg_braintax(self, avg_braintax: f64) -> Self {
        Self {
            avg_braintax,
            ..self
        }
    }

    #[must_use]
    pub fn with_avg_cyclomatic(self, avg_cyclomatic: f64) -> Self {
        Self {
            avg_cyclomatic,
            ..self
        }
    }

    #[must_use]
    pub fn with_braintax_normalized(self, braintax_normalized: u32) -> Self {
        Self {
            braintax_normalized,
            ..self
        }
    }

    #[must_use]
    pub fn with_function_count(self, function_count: usize) -> Self {
        Self {
            function_count,
            ..self
        }
    }

    #[must_use]
    pub fn with_max_braintax(self, max_braintax: f64) -> Self {
        Self {
            max_braintax,
            ..self
        }
    }

    #[must_use]
    pub fn with_max_cyclomatic(self, max_cyclomatic: u32) -> Self {
        Self {
            max_cyclomatic,
            ..self
        }
    }

    #[must_use]
    pub fn with_path(self, path: String) -> Self {
        Self { path, ..self }
    }

    #[must_use]
    pub fn with_total_braintax(self, total_braintax: f64) -> Self {
        Self {
            total_braintax,
            ..self
        }
    }

    #[must_use]
    pub fn with_total_cyclomatic(self, total_cyclomatic: u32) -> Self {
        Self {
            total_cyclomatic,
            ..self
        }
    }
}
