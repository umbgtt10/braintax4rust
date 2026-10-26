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
fn cfg_shows_both_functions() {
    // Arrange & Act
    let report = analyze();

    // Assert
    assert_eq!(report.overall.total_functions, 2);
    assert_eq!(report.overall.max_cyclomatic, 18);
    assert_eq!(report.overall.max_braintax, 36.0);
    assert!(
        report
            .functions
            .iter()
            .any(|f| f.name == "compute" && f.cyclomatic == 18 && f.braintax == 36.0)
    );
    assert!(
        report
            .functions
            .iter()
            .any(|f| f.name == "placeholder" && f.cyclomatic == 1 && f.braintax == 1.0)
    );
}
