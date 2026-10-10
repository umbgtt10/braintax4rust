// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::analysis::fs_walk::FsWalk;
use braintax::invocation::app::App;
use braintax::invocation::config::Config;
use braintax::reporting::braintax_report::BraintaxReport;
use braintax::reporting::default_scorer::DefaultScorer;
use braintax_test_utils::capture_reporter::CaptureReporter;
use serde_json::from_str;
use std::path::PathBuf;
use std::sync::Arc;

fn analyze() -> BraintaxReport {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let reporter = CaptureReporter::new();
    let captured = Arc::clone(&reporter.captured);
    let app = App::with_deps(
        Box::new(FsWalk::new(&path)),
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
    app.run().unwrap();
    let json = captured.lock().unwrap().clone();
    from_str(&json).unwrap()
}

#[test]
fn multi_impl_trait_costs_more_than_single() {
    // Arrange & Act
    let report = analyze();

    // Assert
    assert!(report.overall.total_functions >= 3);
    for func in &report.functions {
        assert_eq!(func.cyclomatic, 18);
        assert!((func.braintax - 18.56).abs() < 0.01);
    }
}
