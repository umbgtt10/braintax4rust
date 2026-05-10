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

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

struct CaptureReporter {
    captured: std::sync::Mutex<String>,
}

impl CaptureReporter {
    fn new() -> Self {
        Self {
            captured: std::sync::Mutex::new(String::new()),
        }
    }
}

impl Reporter for CaptureReporter {
    fn render(&self, report: &BraintaxReport) -> Result<String, anyhow::Error> {
        let json = serde_json::to_string_pretty(report)?;
        let mut guard = self.captured.lock().unwrap();
        *guard = json.clone();
        Ok(json)
    }

    fn write(&self, report: &BraintaxReport) -> Result<(), anyhow::Error> {
        let json = self.render(report)?;
        println!("{json}");
        Ok(())
    }
}

fn analyze_fixture(name: &str) -> BraintaxReport {
    let path = fixture_path(name);
    let config = Config {
        path,
        json: true,
        threshold: None,
        top: 10,
    };
    let reporter = CaptureReporter::new();
    let app: App<FsWalk, DefaultScorer, CaptureReporter> = App::with_deps(
        FsWalk::new(&config.path),
        DefaultScorer::new(),
        reporter,
        config,
    );
    let _ = app.run().unwrap();
    let guard = app.reporter().captured.lock().unwrap();
    serde_json::from_str(&guard).unwrap()
}

#[test]
fn base_flat_has_cc_18() {
    // Arrange & Act
    let report = analyze_fixture("base_flat");

    // Assert
    assert_eq!(report.overall.total_functions, 1);
    assert_eq!(report.overall.max_cyclomatic, 18);
    assert_eq!(report.functions[0].name, "compute");
    assert_eq!(report.functions[0].module, ".");
    assert_eq!(report.functions[0].cyclomatic, 18);
}

#[test]
fn base_depth1_has_cc_18() {
    // Arrange & Act
    let report = analyze_fixture("base_depth1");

    // Assert
    assert_eq!(report.overall.total_functions, 1);
    assert_eq!(report.overall.max_cyclomatic, 18);
    assert_eq!(report.functions[0].name, "compute");
    assert_eq!(report.functions[0].module, "deep");
    assert_eq!(report.functions[0].cyclomatic, 18);
}

#[test]
fn base_cfg_shows_both_functions() {
    // Arrange & Act
    let report = analyze_fixture("base_cfg");

    // Assert
    // #[cfg] is a compiler directive; syn parses both functions regardless
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

#[test]
fn base_trait_has_cc_18_in_impl() {
    // Arrange & Act
    let report = analyze_fixture("base_trait");

    // Assert
    assert_eq!(report.overall.total_functions, 1);
    assert_eq!(report.overall.max_cyclomatic, 18);
    assert_eq!(report.functions[0].name, "compute");
    assert_eq!(report.functions[0].module, ".");
    assert_eq!(report.functions[0].cyclomatic, 18);
}
