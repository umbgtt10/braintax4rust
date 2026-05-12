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
fn multi_impl_trait_costs_more_than_single() {
    // Arrange & Act
    let report = analyze();

    // Assert
    // Processor: 3 impls → dispatch_penalty = (3-1) * 0.06 = 0.12
    // factor = 0.90 + 0.12 = 1.02
    // Each impl fn: CC=18, &self=0.5 → 18 × 1.02 + 0.5 = 18.86
    assert!(report.overall.total_functions >= 3);
    for func in &report.functions {
        assert_eq!(func.cyclomatic, 18);
        assert!((func.braintax - 18.86).abs() < 0.01);
    }
}
