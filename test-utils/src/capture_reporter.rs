// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::sync::Mutex;

use anyhow::Result;
use braintax::braintax_report::BraintaxReport;
use braintax::traits::reporter::Reporter;

pub struct CaptureReporter {
    pub captured: Mutex<String>,
}

impl CaptureReporter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for CaptureReporter {
    fn default() -> Self {
        Self {
            captured: Mutex::new(String::new()),
        }
    }
}

impl Reporter for CaptureReporter {
    fn render(&self, report: &BraintaxReport) -> Result<String> {
        let json = serde_json::to_string_pretty(report)?;
        *self.captured.lock().unwrap() = json.clone();
        Ok(json)
    }

    fn write(&self, report: &BraintaxReport) -> Result<()> {
        let json = self.render(report)?;
        println!("{json}");
        Ok(())
    }
}
