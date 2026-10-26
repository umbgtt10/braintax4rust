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
