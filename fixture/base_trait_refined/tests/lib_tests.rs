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
fn refined_trait_braintax_reflects_complex_trait() {
    // Arrange & Act
    let report = analyze();

    // Assert
    let run_fn = report.functions.iter().find(|f| f.name == "run").unwrap();
    assert_eq!(run_fn.cyclomatic, 18);
    assert!((run_fn.braintax - 25.5).abs() < 0.01);
}
