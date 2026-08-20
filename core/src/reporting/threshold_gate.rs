// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::reporting::overall_stats::OverallStats;

#[derive(Debug, Clone)]
pub struct ThresholdGate {
    max_cyclomatic: Option<u32>,
    max_avg_braintax: Option<f64>,
}

impl ThresholdGate {
    #[must_use]
    pub const fn new(max_cyclomatic: Option<u32>, max_avg_braintax: Option<f64>) -> Self {
        Self {
            max_cyclomatic,
            max_avg_braintax,
        }
    }

    #[must_use]
    pub fn passes(&self, overall: &OverallStats) -> bool {
        self.cyclomatic_passes(overall) && self.avg_braintax_passes(overall)
    }

    fn cyclomatic_passes(&self, overall: &OverallStats) -> bool {
        self.max_cyclomatic
            .is_none_or(|max| overall.max_cyclomatic <= max)
    }

    fn avg_braintax_passes(&self, overall: &OverallStats) -> bool {
        self.max_avg_braintax
            .is_none_or(|max| overall.avg_braintax <= max)
    }
}
