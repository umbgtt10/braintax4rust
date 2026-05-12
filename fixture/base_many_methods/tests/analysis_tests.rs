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
fn many_methods_has_method_penalty() {
    // Arrange & Act
    let report = analyze();

    // Assert
    assert_eq!(report.overall.total_functions, 5);
    let compute_fn = report
        .functions
        .iter()
        .find(|f| f.name == "compute")
        .unwrap();
    assert_eq!(compute_fn.cyclomatic, 18);
    assert!((compute_fn.braintax - 17.06).abs() < 0.01);
    for func in &report.functions {
        if func.name != "compute" {
            assert_eq!(func.cyclomatic, 1);
            assert!((func.braintax - 1.42).abs() < 0.01);
        }
    }
}
