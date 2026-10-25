// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;
use braintax::analysis::fs_walk::FsWalk;
use braintax::invocation::app::App;
use braintax::invocation::config::Config;
use braintax::reporting::braintax_report::BraintaxReport;
use braintax::reporting::default_scorer::DefaultScorer;
use braintax_test_utils::capture_reporter::CaptureReporter;
use serde_json::from_str;
use std::path::PathBuf;

pub struct FixtureAnalyzer {
    fixture_root: PathBuf,
}

impl FixtureAnalyzer {
    pub fn new(fixture_root: PathBuf) -> Self {
        Self { fixture_root }
    }

    pub fn analyze(&self, name: &str) -> Result<BraintaxReport> {
        let path = self.fixture_root.join(name);
        let reporter = CaptureReporter::new();
        let captured = reporter.captured();
        let app = App::new(
            Box::new(FsWalk::new(&path)),
            Box::new(DefaultScorer::new()),
            Box::new(reporter),
            Config::new(path, true, None, None, 10),
        );
        app.run()?;
        let json = captured.lock().unwrap().clone();
        Ok(from_str(&json)?)
    }
}
