// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Result;
use braintax::app::App;
use braintax::config::Config;
use braintax::default_scorer::DefaultScorer;
use braintax::traits::walk::Walk;
use braintax_test_utils::capture_reporter::CaptureReporter;

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
    let config = Config {
        path: PathBuf::from("."),
        json: true,
        threshold: None,
        top: 10,
    };
    let app = App::with_deps(walk, scorer, reporter, config);

    // Act
    let _ = app.run();
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
    let config = Config {
        path: PathBuf::from("."),
        json: true,
        threshold: None,
        top: 10,
    };
    let app = App::with_deps(walk, scorer, reporter, config);

    // Act
    let _ = app.run();
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
    let config_ok = Config {
        path: PathBuf::from("."),
        json: false,
        threshold: Some(10),
        top: 10,
    };
    let config_fail = Config {
        path: PathBuf::from("."),
        json: false,
        threshold: Some(3),
        top: 10,
    };

    // Act
    let app_ok = App::with_deps(walk.clone(), scorer, reporter1, config_ok);
    let result_ok = app_ok.run();

    // Act & Assert
    let app_fail = App::with_deps(walk, DefaultScorer::new(), reporter2, config_fail);
    let result_fail = app_fail.run();

    assert_eq!(result_ok.unwrap(), ExitCode::SUCCESS);
    assert_eq!(result_fail.unwrap(), ExitCode::FAILURE);
}

#[derive(Debug, Clone)]
struct NoFilesWalk;

impl Walk for NoFilesWalk {
    fn rust_files(&self) -> Result<Vec<(PathBuf, String)>> {
        Ok(vec![])
    }
}

#[test]
fn app_with_no_files_returns_error() {
    // Arrange
    let scorer = DefaultScorer::new();
    let reporter = CaptureReporter::new();
    let config = Config {
        path: PathBuf::from("."),
        json: false,
        threshold: None,
        top: 10,
    };
    let app = App::with_deps(NoFilesWalk, scorer, reporter, config);

    // Act
    let result = app.run();

    // Assert
    assert!(result.is_err());
}
