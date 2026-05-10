// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use braintax::app::App;
use braintax::braintax_report::BraintaxReport;
use braintax::config::Config;
use braintax::default_scorer::DefaultScorer;
use braintax::fs_walk::FsWalk;
use braintax::traits::reporter::Reporter;

struct CaptureReporter {
    captured: std::sync::Mutex<String>,
}

impl Reporter for CaptureReporter {
    fn render(&self, report: &BraintaxReport) -> Result<String, anyhow::Error> {
        Ok(serde_json::to_string_pretty(report)?)
    }
    fn write(&self, report: &BraintaxReport) -> Result<(), anyhow::Error> {
        let json = serde_json::to_string_pretty(report)?;
        *self.captured.lock().unwrap() = json;
        Ok(())
    }
}

fn analyze() -> BraintaxReport {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("base_cfg");
    let reporter = CaptureReporter {
        captured: std::sync::Mutex::new(String::new()),
    };
    let app = App::with_deps(
        FsWalk::new(&path),
        DefaultScorer::new(),
        reporter,
        Config {
            path,
            json: true,
            threshold: None,
            top: 10,
        },
    );
    app.run().unwrap();
    let json = app.reporter().captured.lock().unwrap().clone();
    serde_json::from_str(&json).unwrap()
}

#[test]
fn cfg_shows_both_functions() {
    // Arrange & Act
    let report = analyze();

    // Assert
    assert_eq!(report.overall.total_functions, 2);
    assert_eq!(report.overall.max_cyclomatic, 18);
    assert!(
        report
            .functions
            .iter()
            .any(|f| f.name == "compute" && f.cyclomatic == 18)
    );
    assert!(
        report
            .functions
            .iter()
            .any(|f| f.name == "placeholder" && f.cyclomatic == 1)
    );
}
