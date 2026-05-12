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

fn analyze_fixture(name: &str) -> BraintaxReport {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("fixture")
        .join(name);
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
fn fixture_braintax_ordinal_ranking() {
    // Arrange
    let fixtures: &[(&str, f64)] = &[
        ("base_trait", 16.7),
        ("base_known_trait", 16.9),
        ("base_many_methods", 17.06),
        ("base_inherent", 17.6),
        ("base_trait_two_impls", 17.78),
        ("base_flat", 18.0),
        ("base_trait_multi", 18.86),
        ("base_trait_four_impls", 19.94),
        ("base_depth1", 20.7),
        ("base_assoc_only", 23.9),
        ("base_super_only", 23.9),
        ("base_macros", 24.0),
        ("base_opaque", 24.0),
        ("base_trait_refined", 25.8),
        ("base_trait_assoc_dispatch", 27.14),
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
