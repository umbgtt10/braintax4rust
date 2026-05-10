// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::BTreeMap;

use crate::function_complexity::FunctionComplexity;
use crate::module_stats::ModuleStats;
use crate::overall_stats::OverallStats;
use crate::traits::scorer::Scorer;

#[derive(Debug, Clone, Default)]
pub struct DefaultScorer;

impl DefaultScorer {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Scorer for DefaultScorer {
    fn overall_stats(&self, functions: &[FunctionComplexity]) -> OverallStats {
        let count = functions.len();
        if count == 0 {
            return OverallStats {
                total_functions: 0,
                avg_cyclomatic: 0.0,
                max_cyclomatic: 0,
                total_cyclomatic: 0,
            };
        }

        let total: u32 = functions.iter().map(|f| f.cyclomatic).sum();
        let max = functions.iter().map(|f| f.cyclomatic).max().unwrap_or(0);
        let avg = total as f64 / count as f64;

        OverallStats {
            total_functions: count,
            avg_cyclomatic: avg,
            max_cyclomatic: max,
            total_cyclomatic: total,
        }
    }

    fn module_stats(&self, functions: &[FunctionComplexity]) -> Vec<ModuleStats> {
        let mut module_map: BTreeMap<String, Vec<&FunctionComplexity>> = BTreeMap::new();
        for func in functions {
            module_map
                .entry(func.module.clone())
                .or_default()
                .push(func);
        }

        module_map
            .into_iter()
            .map(|(path, funcs)| {
                let count = funcs.len();
                let total: u32 = funcs.iter().map(|f| f.cyclomatic).sum();
                let max = funcs.iter().map(|f| f.cyclomatic).max().unwrap_or(0);
                ModuleStats {
                    path,
                    function_count: count,
                    avg_cyclomatic: if count > 0 {
                        total as f64 / count as f64
                    } else {
                        0.0
                    },
                    max_cyclomatic: max,
                    total_cyclomatic: total,
                }
            })
            .collect()
    }
}
