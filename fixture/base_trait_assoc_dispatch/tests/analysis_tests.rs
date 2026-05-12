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
fn assoc_type_amplifies_dispatch_penalty() {
    // Arrange & Act
    let report = analyze();

    // Assert
    // Processor: 1 method, 1 assoc, 0 super, 3 impls
    // base = 1.15, dim_penalty = 0.15 (assoc only)
    // dispatch_base = min((3-1) × 0.06, 0.18) = 0.12
    // dispatch_amplifier = 1.5 (assoc present)
    // dispatch = min(0.12 × 1.5, 0.27) = 0.18
    // factor = 1.15 + 0.15 + 0.18 = 1.48
    // braintax = 18 × 1.48 + 0.5 = 27.14
    assert!(report.overall.total_functions >= 3);
    for func in &report.functions {
        assert_eq!(func.cyclomatic, 18);
        assert!((func.braintax - 27.14).abs() < 0.01);
    }
}
