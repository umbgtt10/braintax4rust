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

pub fn compute_braintax_impl(
    cfg_gates: u32,
    cyclomatic: u32,
    hidden_deps: u32,
    depth: u32,
    trait_factor: f64,
) -> f64 {
    let cfg_factor = if cfg_gates > 0 {
        2.0f64.powi(cfg_gates as i32)
    } else {
        1.0
    };
    let depth_factor = 1.0 + (depth.saturating_sub(1) as f64) * 0.15;
    let hidden_penalty = hidden_deps as f64 * 4.0;
    cyclomatic as f64 * cfg_factor * depth_factor * trait_factor + hidden_penalty
}

impl DefaultScorer {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    pub fn compute_braintax(func: &FunctionComplexity) -> f64 {
        compute_braintax_impl(
            func.cfg_gates,
            func.cyclomatic,
            func.hidden_deps,
            func.depth,
            func.trait_factor,
        )
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
                avg_braintax: 0.0,
                max_braintax: 0.0,
            };
        }

        let total_cc: u32 = functions.iter().map(|f| f.cyclomatic).sum();
        let max_cc = functions.iter().map(|f| f.cyclomatic).max().unwrap_or(0);
        let avg_cc = total_cc as f64 / count as f64;

        let max_bt = functions
            .iter()
            .map(|f| f.braintax)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
        let avg_bt = functions.iter().map(|f| f.braintax).sum::<f64>() / count as f64;

        OverallStats {
            total_functions: count,
            avg_cyclomatic: avg_cc,
            max_cyclomatic: max_cc,
            total_cyclomatic: total_cc,
            avg_braintax: avg_bt,
            max_braintax: max_bt,
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
                let total_cc: u32 = funcs.iter().map(|f| f.cyclomatic).sum();
                let max_cc = funcs.iter().map(|f| f.cyclomatic).max().unwrap_or(0);
                let avg_cc = if count > 0 {
                    total_cc as f64 / count as f64
                } else {
                    0.0
                };
                let max_bt = funcs
                    .iter()
                    .map(|f| f.braintax)
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(0.0);
                let avg_bt = funcs.iter().map(|f| f.braintax).sum::<f64>() / count as f64;

                ModuleStats {
                    path,
                    function_count: count,
                    avg_cyclomatic: avg_cc,
                    max_cyclomatic: max_cc,
                    total_cyclomatic: total_cc,
                    avg_braintax: avg_bt,
                    max_braintax: max_bt,
                }
            })
            .collect()
    }
}
