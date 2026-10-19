// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::BTreeMap;

use crate::counting::function_complexity::FunctionComplexity;
use crate::reporting::braintax_components::BraintaxComponents;
use crate::reporting::braintax_normalizer::BraintaxNormalizer;
use crate::reporting::module_stats::ModuleStats;
use crate::reporting::overall_stats::OverallStats;
use crate::traits::scorer::Scorer;

#[derive(Debug, Clone, Default)]
pub struct DefaultScorer {
    normalizer: BraintaxNormalizer,
}

impl DefaultScorer {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            normalizer: BraintaxNormalizer::new(),
        }
    }

    pub fn compute_braintax(func: &FunctionComplexity) -> f64 {
        BraintaxComponents {
            cfg_gates: func.cfg_gates,
            cyclomatic: func.cyclomatic,
            hidden_dep_weight: func.hidden_dep_weight,
            depth: func.depth,
            trait_factor: func.trait_factor,
            name_opacity: 0,
            macro_density: 0,
            generics: 0,
            self_ref_cost: 0.0,
            return_complexity: 0.0,
        }
        .compute_braintax()
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
                total_braintax: 0.0,
                braintax_normalized: 0,
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
        let total_bt: f64 = functions.iter().map(|f| f.braintax).sum();
        let avg_bt = total_bt / count as f64;

        OverallStats {
            total_functions: count,
            avg_cyclomatic: avg_cc,
            max_cyclomatic: max_cc,
            total_cyclomatic: total_cc,
            avg_braintax: avg_bt,
            max_braintax: max_bt,
            total_braintax: total_bt,
            braintax_normalized: self.normalizer.normalize(avg_bt),
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
                let total_bt: f64 = funcs.iter().map(|f| f.braintax).sum();
                let avg_bt = total_bt / count as f64;

                ModuleStats {
                    path,
                    function_count: count,
                    avg_cyclomatic: avg_cc,
                    max_cyclomatic: max_cc,
                    total_cyclomatic: total_cc,
                    avg_braintax: avg_bt,
                    max_braintax: max_bt,
                    total_braintax: total_bt,
                    braintax_normalized: self.normalizer.normalize(avg_bt),
                }
            })
            .collect()
    }
}
