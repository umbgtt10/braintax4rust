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
        // 1: simple trait 1 impl (0.90) + self(0.5)
        ("base_trait", 16.7),
        // 2: known std (0.80) + self(0.5) + name_opacity f(2)
        ("base_known_trait", 16.9),
        // 3: 5 methods method_penalty (0.92) + self(0.5)
        ("base_many_methods", 17.06),
        // 4: inherent impl (0.95) + self(0.5)
        ("base_inherent", 17.6),
        // 5: 2 impls dispatch (0.96) + self(0.5)
        ("base_trait_two_impls", 17.78),
        // 6: free function baseline (1.0)
        ("base_flat", 18.0),
        // 7: 3 impls dispatch (1.02) + self(0.5)
        ("base_trait_multi", 18.86),
        // 8: 4 impls dispatch capped (1.08) + self(0.5)
        ("base_trait_four_impls", 19.94),
        // 9: depth=2 -> factor 1.15
        ("base_depth1", 20.7),
        // 10: assoc only (1.30) + self(0.5)
        ("base_assoc_only", 23.9),
        // 11: super only (1.30) + self(0.5)
        ("base_super_only", 23.9),
        // 12: macros 2*3=+6 to baseline
        ("base_macros", 24.0),
        // 13: opaque names a(2)+b(2)+c(2)=+6 to baseline
        ("base_opaque", 24.0),
        // 14: assoc+super (1.35) + self(0.5) + ctx(1)
        ("base_trait_refined", 25.8),
        // 15: assoc+3impls (1.48) + self(0.5)
        ("base_trait_assoc_dispatch", 27.14),
        // 16: generics=9 + name_opacity=3
        ("base_generics", 30.0),
        // 17: cfg gate 2^1=2.0
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
