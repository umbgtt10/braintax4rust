// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;
use braintax::invocation::app::App;
use braintax::invocation::config::Config;
use braintax::reporting::braintax_report::BraintaxReport;
use braintax::reporting::default_scorer::DefaultScorer;
use braintax::traits::walk::Walk;
use braintax_test_utils::capture_reporter::CaptureReporter;
use serde_json::from_str;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Debug, Clone)]
struct NoFilesWalk;

impl Walk for NoFilesWalk {
    fn rust_files(&self) -> Result<Vec<(PathBuf, String)>> {
        Ok(vec![])
    }
}

#[derive(Debug, Clone)]
struct TestWalk {
    files: Vec<(PathBuf, String)>,
}

impl Walk for TestWalk {
    fn rust_files(&self) -> Result<Vec<(PathBuf, String)>> {
        Ok(self.files.clone())
    }
}

#[test]
fn app_with_if_fn_returns_cc_2() {
    // Arrange
    let source = r#"
fn with_if(x: bool) {
    if x {
        let _ = 1;
    }
}
"#
    .to_string();
    let walk = TestWalk {
        files: vec![(PathBuf::from("src/lib.rs"), source)],
    };
    let scorer = DefaultScorer::new();
    let reporter = CaptureReporter::new();
    let config = Config::new(PathBuf::from("."), true, None, None, 10);
    let captured = reporter.captured();
    let app = App::new(Box::new(walk), Box::new(scorer), Box::new(reporter), config);

    // Act
    let result = app.run();

    // Assert
    assert_eq!(result.unwrap(), ExitCode::SUCCESS);
    let report: BraintaxReport = from_str(&captured.lock().unwrap()).expect("report json");
    assert_eq!(report.overall.max_cyclomatic, 2);
}

#[test]
fn app_with_no_files_returns_error() {
    // Arrange
    let scorer = DefaultScorer::new();
    let reporter = CaptureReporter::new();
    let config = Config::new(PathBuf::from("."), false, None, None, 10);
    let app = App::new(
        Box::new(NoFilesWalk),
        Box::new(scorer),
        Box::new(reporter),
        config,
    );

    // Act
    let result = app.run();

    // Assert
    assert!(result.is_err());
}

#[test]
fn app_with_threshold_returns_exit_code() {
    // Arrange
    let source = r#"
fn complex(a: bool, b: bool) {
    if a && b {
        loop {
            break;
        }
    }
}
"#
    .to_string();
    let walk = TestWalk {
        files: vec![(PathBuf::from("src/lib.rs"), source)],
    };
    let scorer = DefaultScorer::new();
    let reporter1 = CaptureReporter::new();
    let reporter2 = CaptureReporter::new();
    let config_ok = Config::new(PathBuf::from("."), false, Some(10), None, 10);
    let config_fail = Config::new(PathBuf::from("."), false, Some(3), None, 10);

    // Act
    let app_ok = App::new(
        Box::new(walk.clone()),
        Box::new(scorer),
        Box::new(reporter1),
        config_ok,
    );
    let result_ok = app_ok.run();

    // Assert
    assert_eq!(result_ok.unwrap(), ExitCode::SUCCESS);

    // Act
    let app_fail = App::new(
        Box::new(walk),
        Box::new(DefaultScorer::new()),
        Box::new(reporter2),
        config_fail,
    );
    let result_fail = app_fail.run();

    // Assert
    assert_eq!(result_fail.unwrap(), ExitCode::FAILURE);
}

#[test]
fn app_with_trivial_fn_returns_cc_1() {
    // Arrange
    let source = r#"
fn trivial() {
    let x = 1;
}
"#
    .to_string();
    let walk = TestWalk {
        files: vec![(PathBuf::from("src/lib.rs"), source)],
    };
    let scorer = DefaultScorer::new();
    let reporter = CaptureReporter::new();
    let config = Config::new(PathBuf::from("."), true, None, None, 10);
    let captured = reporter.captured();
    let app = App::new(Box::new(walk), Box::new(scorer), Box::new(reporter), config);

    // Act
    let result = app.run();

    // Assert
    assert_eq!(result.unwrap(), ExitCode::SUCCESS);
    let report: BraintaxReport = from_str(&captured.lock().unwrap()).expect("report json");
    assert_eq!(report.overall.max_cyclomatic, 1);
}

#[test]
fn run_with_a_threshold_set_still_writes_the_report() {
    // Arrange
    let walk = TestWalk {
        files: vec![(PathBuf::from("src/lib.rs"), "fn trivial() {}".to_string())],
    };
    let reporter = CaptureReporter::new();
    let captured = reporter.captured();
    let config = Config::new(PathBuf::from("."), false, Some(1), None, 10);
    let app = App::new(
        Box::new(walk),
        Box::new(DefaultScorer::new()),
        Box::new(reporter),
        config,
    );

    // Act
    let result = app.run();

    // Assert
    assert_eq!(result.unwrap(), ExitCode::SUCCESS);
    assert!(!captured.lock().unwrap().is_empty());
}

#[test]
fn run_with_avg_braintax_above_the_limit_returns_failure() {
    // Arrange
    let walk = TestWalk {
        files: vec![(PathBuf::from("src/lib.rs"), "fn trivial() {}".to_string())],
    };
    let config = Config::new(PathBuf::from("."), false, None, Some(0.0), 10);
    let app = App::new(
        Box::new(walk),
        Box::new(DefaultScorer::new()),
        Box::new(CaptureReporter::new()),
        config,
    );

    // Act
    let result = app.run();

    // Assert
    assert_eq!(result.unwrap(), ExitCode::FAILURE);
}

#[test]
fn run_with_avg_braintax_within_the_limit_returns_success() {
    // Arrange
    let walk = TestWalk {
        files: vec![(PathBuf::from("src/lib.rs"), "fn trivial() {}".to_string())],
    };
    let config = Config::new(PathBuf::from("."), false, None, Some(100.0), 10);
    let app = App::new(
        Box::new(walk),
        Box::new(DefaultScorer::new()),
        Box::new(CaptureReporter::new()),
        config,
    );

    // Act
    let result = app.run();

    // Assert
    assert_eq!(result.unwrap(), ExitCode::SUCCESS);
}
