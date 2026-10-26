// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;
use braintax::analysis::source_walk::SourceWalk;
use braintax::invocation::app::App;
use braintax::invocation::config::Config;
use braintax::reporting::braintax_report::BraintaxReport;
use braintax::reporting::default_scorer::DefaultScorer;
use braintax_cli::system::portable_path::PortablePath;
use braintax_cli::system::walkdir_source_tree::WalkdirSourceTree;
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
        let path = PortablePath::of(&self.fixture_root.join(name));
        let reporter = CaptureReporter::new();
        let captured = reporter.captured();
        let app = App::new(
            Box::new(SourceWalk::new(
                Box::new(WalkdirSourceTree::new()),
                path.clone(),
            )),
            Box::new(DefaultScorer::new()),
            Box::new(reporter),
            Config {
                path,
                json: true,
                threshold: None,
                max_avg_braintax: None,
                top: 10,
            },
        );
        app.run()?;
        let json = captured.lock().unwrap().clone();
        Ok(from_str(&json)?)
    }
}
