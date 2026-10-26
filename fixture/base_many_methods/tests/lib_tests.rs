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
fn many_methods_has_method_penalty() {
    // Arrange & Act
    let report = analyze();

    // Assert
    assert_eq!(report.overall.total_functions, 5);
    let compute_fn = report
        .functions
        .iter()
        .find(|f| f.name == "compute")
        .unwrap();
    assert_eq!(compute_fn.cyclomatic, 18);
    assert!((compute_fn.braintax - 16.76).abs() < 0.01);
    for func in &report.functions {
        if func.name != "compute" {
            assert_eq!(func.cyclomatic, 1);
            assert!((func.braintax - 1.12).abs() < 0.01);
        }
    }
}
