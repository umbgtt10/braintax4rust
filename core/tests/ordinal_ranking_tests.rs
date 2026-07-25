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

fn analyze_fixture(name: &str) -> BraintaxReport {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("fixture")
        .join(name);
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
fn fixture_braintax_ordinal_ranking() {
    // Arrange
    let fixtures: &[(&str, f64)] = &[
        ("base_hidden_deps", 16.0),
        ("base_trait", 16.4),
        ("base_known_trait", 16.6),
        ("base_many_methods", 16.76),
        ("base_inherent", 17.3),
        ("base_trait_two_impls", 17.48),
        ("base_flat", 18.0),
        ("base_trait_multi", 18.56),
        ("base_trait_four_impls", 19.64),
        ("base_depth1", 20.7),
        ("base_assoc_only", 23.6),
        ("base_super_only", 23.6),
        ("base_macros", 24.0),
        ("base_opaque", 24.0),
        ("base_trait_refined", 25.5),
        ("base_trait_assoc_dispatch", 26.84),
        ("base_generics", 30.0),
        ("base_cfg", 36.0),
    ];

    // Act & Assert
    let mut prev_braintax: Option<f64> = None;
    let mut prev_name: Option<&str> = None;
    for &(name, expected) in fixtures {
        let report = analyze_fixture(name);
        let got = report.overall.max_braintax;

        assert!(
            (got - expected).abs() < 0.01,
            "{name}: expected max_braintax {expected}, got {got}",
        );

        if let (Some(prev), Some(prev_n)) = (prev_braintax, prev_name) {
            assert!(
                got + 0.001 >= prev,
                "{name}: braintax {got} < {prev} from {prev_n} - ranking violated",
            );
        }
        prev_braintax = Some(got);
        prev_name = Some(name);
    }
}
