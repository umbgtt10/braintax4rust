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
fn trait_cc_is_18() {
    // Arrange & Act
    let report = analyze();

    // Assert
    assert_eq!(report.overall.total_functions, 1);
    assert_eq!(report.overall.max_cyclomatic, 18);
    assert_eq!(report.overall.max_braintax, 16.4);
    assert_eq!(report.functions[0].name, "compute");
    assert_eq!(report.functions[0].cyclomatic, 18);
    assert!((report.functions[0].braintax - 16.4).abs() < 0.01);
}
