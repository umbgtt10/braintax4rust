// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

use crate::reporting::braintax_report::BraintaxReport;
use crate::reporting::render_failure::RenderFailure;

pub trait ReportRenderer {
    fn render(&self, report: &BraintaxReport) -> Result<String, RenderFailure>;
}
