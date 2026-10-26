// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::reporting::braintax_report::BraintaxReport;

// What one run found: the report, the text its renderer made of it, and
// whether it stays within the configured ceilings.
#[derive(Debug, Clone)]
pub struct RunOutcome {
    pub report: BraintaxReport,
    pub output: String,
    pub passes: bool,
}
