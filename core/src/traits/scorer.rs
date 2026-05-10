// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::function_complexity::FunctionComplexity;
use crate::module_stats::ModuleStats;
use crate::overall_stats::OverallStats;

pub trait Scorer {
    fn overall_stats(&self, functions: &[FunctionComplexity]) -> OverallStats;
    fn module_stats(&self, functions: &[FunctionComplexity]) -> Vec<ModuleStats>;
}
