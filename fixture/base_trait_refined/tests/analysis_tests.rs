// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use braintax::app::App;
use braintax::braintax_report::BraintaxReport;
use braintax::config::Config;
use braintax::default_scorer::DefaultScorer;
use braintax::fs_walk::FsWalk;
use braintax_test_utils::capture_reporter::CaptureReporter;

fn analyze() -> BraintaxReport {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let reporter = CaptureReporter::new();
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
fn refined_trait_braintax_reflects_complex_trait() {
    // Arrange & Act
    let report = analyze();

    // Assert
    // CC=18, Extended factor: 1.15 + 0.10(assoc) + 0.10(super) = 1.35
    // name_opacity: ctx(3ch) = 1
    // braintax = 18 × 1.35 + 1 = 25.3
    let run_fn = report.functions.iter().find(|f| f.name == "run").unwrap();
    assert_eq!(run_fn.cyclomatic, 18);
    assert!((run_fn.braintax - 25.3).abs() < 0.01);
}
