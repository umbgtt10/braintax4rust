// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_source_tree::NotHeld;
use braintax::reporting::braintax_report::BraintaxReport;
use braintax::reporting::render_failure::RenderFailure;
use braintax::traits::report_renderer::ReportRenderer;

// A renderer that writes nothing a reader would want: it answers with the
// report's target and function count, or fails.
pub struct StubReportRenderer {
    fails: bool,
}

impl StubReportRenderer {
    pub fn new() -> Self {
        Self { fails: false }
    }

    pub fn failing() -> Self {
        Self { fails: true }
    }
}

impl Default for StubReportRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportRenderer for StubReportRenderer {
    fn render(&self, report: &BraintaxReport) -> Result<String, RenderFailure> {
        if self.fails {
            return Err(RenderFailure {
                cause: Box::new(NotHeld),
            });
        }
        Ok(format!(
            "{}: {} functions",
            report.target, report.overall.total_functions
        ))
    }
}
