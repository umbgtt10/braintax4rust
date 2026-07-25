// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;
use std::sync::Arc;

use braintax::app::App;
use braintax::braintax_report::BraintaxReport;
use braintax::config::Config;
use braintax::default_scorer::DefaultScorer;
use braintax::fs_walk::FsWalk;
use braintax_test_utils::capture_reporter::CaptureReporter;

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
            top: 10,
        },
    );
    app.run().unwrap();
    let json = captured.lock().unwrap().clone();
    serde_json::from_str(&json).unwrap()
}

#[test]
fn clean_fn_has_no_hidden_deps() {
    // Arrange & Act
    let report = analyze();

    // Assert
    let clean_fn = report.functions.iter().find(|f| f.name == "clean").unwrap();
    assert_eq!(clean_fn.hidden_deps, 0);
    assert_eq!(clean_fn.hidden_dep_weight, 0.0);
    assert!(clean_fn.hidden_dep_labels.is_empty());
    assert_eq!(clean_fn.braintax, 2.0);
}

#[test]
fn risky_fn_sums_severity_weighted_hidden_deps() {
    // Arrange & Act
    let report = analyze();

    // Assert
    let risky_fn = report.functions.iter().find(|f| f.name == "risky").unwrap();
    assert_eq!(risky_fn.hidden_deps, 3);
    assert_eq!(risky_fn.hidden_dep_weight, 14.0);
    assert_eq!(
        risky_fn.hidden_dep_labels,
        vec![
            "unsafe".to_string(),
            "println".to_string(),
            "Instant::now".to_string()
        ]
    );
}

#[test]
fn risky_fn_braintax_reflects_hidden_dep_weight() {
    // Arrange & Act
    let report = analyze();

    // Assert
    let risky_fn = report.functions.iter().find(|f| f.name == "risky").unwrap();
    let clean_fn = report.functions.iter().find(|f| f.name == "clean").unwrap();
    assert_eq!(risky_fn.braintax, 16.0);
    assert_eq!(
        risky_fn.braintax - clean_fn.braintax,
        risky_fn.hidden_dep_weight
    );
}

#[test]
fn overall_max_braintax_is_risky_fn() {
    // Arrange & Act
    let report = analyze();

    // Assert
    assert_eq!(report.overall.total_functions, 2);
    assert_eq!(report.overall.max_braintax, 16.0);
}
