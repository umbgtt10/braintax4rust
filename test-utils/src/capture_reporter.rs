// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::sync::{Arc, Mutex};

use anyhow::Result;
use braintax::reporting::braintax_report::BraintaxReport;
use braintax::traits::reporter::Reporter;
use serde_json::to_string_pretty;

pub struct CaptureReporter {
    captured: Arc<Mutex<String>>,
}

impl CaptureReporter {
    pub fn new() -> Self {
        Self::default()
    }

    // A handle on what the reporter captures, kept by a test that hands the reporter itself away.
    pub fn captured(&self) -> Arc<Mutex<String>> {
        Arc::clone(&self.captured)
    }
}

impl Default for CaptureReporter {
    fn default() -> Self {
        Self {
            captured: Arc::new(Mutex::new(String::new())),
        }
    }
}

impl Reporter for CaptureReporter {
    fn render(&self, report: &BraintaxReport) -> Result<String> {
        let json = to_string_pretty(report)?;
        *self.captured.lock().unwrap() = json.clone();
        Ok(json)
    }

    fn write(&self, report: &BraintaxReport) -> Result<()> {
        let json = self.render(report)?;
        println!("{json}");
        Ok(())
    }
}
