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
fn four_impls_cost_more_than_two() {
    // Arrange & Act
    let report = analyze();

    // Assert
    // Processor: 4 impls → dispatch = min((4-1) × 0.06, 0.18) = 0.18
    // factor = 0.90 + 0.18 = 1.08
    // Each impl fn: CC=18, &self=0.5 → 18 × 1.08 + 0.5 = 19.94
    assert!(report.overall.total_functions >= 4);
    for func in &report.functions {
        assert_eq!(func.cyclomatic, 18);
        assert!((func.braintax - 19.94).abs() < 0.01);
    }
}
