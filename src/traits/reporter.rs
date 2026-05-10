// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;

use crate::braintax_report::BraintaxReport;

pub trait Reporter {
    fn render(&self, report: &BraintaxReport) -> Result<String>;
    fn write(&self, report: &BraintaxReport) -> Result<()>;
}
