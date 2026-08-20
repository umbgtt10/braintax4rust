// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::counting::function_complexity::FunctionComplexity;
use crate::reporting::module_stats::ModuleStats;
use crate::reporting::overall_stats::OverallStats;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BraintaxReport {
    pub version: String,
    pub target: String,
    pub overall: OverallStats,
    pub modules: Vec<ModuleStats>,
    pub functions: Vec<FunctionComplexity>,
}
