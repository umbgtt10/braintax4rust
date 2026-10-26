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
fn flat_cc_is_18() {
    // Arrange & Act
    let report = analyze();

    // Assert
    assert_eq!(report.overall.total_functions, 1);
    assert_eq!(report.overall.max_cyclomatic, 18);
    assert_eq!(report.overall.max_braintax, 18.0);
    assert_eq!(report.functions[0].name, "compute");
    assert_eq!(report.functions[0].module, ".");
    assert_eq!(report.functions[0].cyclomatic, 18);
    assert_eq!(report.functions[0].braintax, 18.0);
}
