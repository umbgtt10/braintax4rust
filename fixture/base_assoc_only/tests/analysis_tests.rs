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
fn assoc_only_costs_more_than_simple_trait() {
    // Arrange & Act
    let report = analyze();

    // Assert
    // Processor: 1 method, 1 assoc, 0 super, 1 impl
    // base = 1.15, dim_penalty = 0.15 → factor = 1.30
    // CC=18, &self=0.5 → 18 × 1.30 + 0.5 = 23.9
    assert_eq!(report.overall.total_functions, 1);
    assert_eq!(report.functions[0].cyclomatic, 18);
    assert!((report.functions[0].braintax - 23.9).abs() < 0.01);
}
