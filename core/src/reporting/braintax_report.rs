// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::counting::function_complexity::FunctionComplexity;
use crate::reporting::module_stats::ModuleStats;
use crate::reporting::overall_stats::OverallStats;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BraintaxReport {
    version: String,
    target: String,
    overall: OverallStats,
    modules: Vec<ModuleStats>,
    functions: Vec<FunctionComplexity>,
}

impl BraintaxReport {
    #[must_use]
    pub fn new(
        version: String,
        target: String,
        overall: OverallStats,
        modules: Vec<ModuleStats>,
        functions: Vec<FunctionComplexity>,
    ) -> Self {
        Self {
            version,
            target,
            overall,
            modules,
            functions,
        }
    }

    pub fn functions(&self) -> &[FunctionComplexity] {
        &self.functions
    }

    pub fn modules(&self) -> &[ModuleStats] {
        &self.modules
    }

    pub fn overall(&self) -> &OverallStats {
        &self.overall
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}
