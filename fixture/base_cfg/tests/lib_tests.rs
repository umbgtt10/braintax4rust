// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::analysis::source_walk::SourceWalk;
use braintax::invocation::app::App;
use braintax::invocation::config::Config;
use braintax::reporting::braintax_report::BraintaxReport;
use braintax::reporting::default_scorer::DefaultScorer;
use braintax_cli::system::portable_path::PortablePath;
use braintax_cli::system::walkdir_source_tree::WalkdirSourceTree;
use braintax_test_utils::capture_reporter::CaptureReporter;
use serde_json::from_str;
use std::path::Path;

fn analyze() -> BraintaxReport {
    let path = PortablePath::of(Path::new(env!("CARGO_MANIFEST_DIR")));
    let reporter = CaptureReporter::new();
    let captured = reporter.captured();
    let app = App::new(
        Box::new(SourceWalk::new(
            Box::new(WalkdirSourceTree::new()),
            path.clone(),
        )),
        Box::new(DefaultScorer::new()),
        Box::new(reporter),
        Config {
            path,
            json: true,
            threshold: None,
            max_avg_braintax: None,
            top: 10,
        },
    );
    app.run().unwrap();
    let json = captured.lock().unwrap().clone();
    from_str(&json).unwrap()
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
