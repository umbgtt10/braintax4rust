// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::invocation::config::Config;
use braintax::reporting::braintax_report::BraintaxReport;
use braintax_cli::invocation::entry_point::EntryPoint;
use braintax_cli::system::portable_path::PortablePath;
use std::path::Path;

fn analyze() -> BraintaxReport {
    let path = PortablePath::of(Path::new(env!("CARGO_MANIFEST_DIR")));
    EntryPoint::compose(Config {
        path,
        json: true,
        threshold: None,
        max_avg_braintax: None,
        top: 10,
    })
    .run()
    .unwrap()
    .report
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
fn overall_max_braintax_is_risky_fn() {
    // Arrange & Act
    let report = analyze();

    // Assert
    assert_eq!(report.overall.total_functions, 2);
    assert_eq!(report.overall.max_braintax, 16.0);
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
