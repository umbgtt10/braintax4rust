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
fn generics_braintax_reflects_generic_penalty() {
    // Arrange & Act
    let report = analyze();

    // Assert
    // CC=18, generics: A(2+1+1) + B(2+1) + E(2) = 9, name_opacity: _a+_b+_e = 3
    // braintax = 18 + 9 + 3 = 30.0
    assert_eq!(report.overall.total_functions, 1);
    assert_eq!(report.functions[0].cyclomatic, 18);
    assert_eq!(report.functions[0].braintax, 30.0);
}
