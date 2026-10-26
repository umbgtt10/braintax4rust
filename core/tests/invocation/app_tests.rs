// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_source_tree::InMemorySourceTree;
use crate::support::stub_report_renderer::StubReportRenderer;
use braintax::analysis::source_path::SourcePath;
use braintax::analysis::source_read_failure::SourceReadFailure;
use braintax::analysis::source_walk::SourceWalk;
use braintax::invocation::app::App;
use braintax::invocation::config::Config;
use braintax::reporting::default_scorer::DefaultScorer;
use braintax::traits::walk::Walk;

#[derive(Debug, Clone)]
struct NoFilesWalk;

impl Walk for NoFilesWalk {
    fn rust_files(&self) -> Result<Vec<(SourcePath, String)>, SourceReadFailure> {
        Ok(vec![])
    }
}

fn path(text: &str) -> SourcePath {
    SourcePath::new(String::from(text))
}

#[derive(Debug, Clone)]
struct TestWalk {
    files: Vec<(SourcePath, String)>,
}

impl Walk for TestWalk {
    fn rust_files(&self) -> Result<Vec<(SourcePath, String)>, SourceReadFailure> {
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
        files: vec![(path("src/lib.rs"), source)],
    };
    let scorer = DefaultScorer::new();
    let renderer = StubReportRenderer::new();
    let config = Config {
        path: path("."),
        json: true,
        threshold: None,
        max_avg_braintax: None,
        top: 10,
    };
    let app = App::new(Box::new(walk), Box::new(scorer), Box::new(renderer), config);

    // Act
    let result = app.run();

    // Assert
    let outcome = result.unwrap();
    assert!(outcome.passes);
    assert_eq!(outcome.report.overall.max_cyclomatic, 2);
}

#[test]
fn app_with_no_files_returns_error() {
    // Arrange
    let scorer = DefaultScorer::new();
    let renderer = StubReportRenderer::new();
    let config = Config {
        path: path("."),
        json: false,
        threshold: None,
        max_avg_braintax: None,
        top: 10,
    };
    let app = App::new(
        Box::new(NoFilesWalk),
        Box::new(scorer),
        Box::new(renderer),
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
        files: vec![(path("src/lib.rs"), source)],
    };
    let scorer = DefaultScorer::new();
    let renderer1 = StubReportRenderer::new();
    let renderer2 = StubReportRenderer::new();
    let config_ok = Config {
        path: path("."),
        json: false,
        threshold: Some(10),
        max_avg_braintax: None,
        top: 10,
    };
    let config_fail = Config {
        path: path("."),
        json: false,
        threshold: Some(3),
        max_avg_braintax: None,
        top: 10,
    };

    // Act
    let app_ok = App::new(
        Box::new(walk.clone()),
        Box::new(scorer),
        Box::new(renderer1),
        config_ok,
    );
    let result_ok = app_ok.run();

    // Assert
    assert!(result_ok.unwrap().passes);

    // Act
    let app_fail = App::new(
        Box::new(walk),
        Box::new(DefaultScorer::new()),
        Box::new(renderer2),
        config_fail,
    );
    let result_fail = app_fail.run();

    // Assert
    assert!(!result_fail.unwrap().passes);
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
        files: vec![(path("src/lib.rs"), source)],
    };
    let scorer = DefaultScorer::new();
    let renderer = StubReportRenderer::new();
    let config = Config {
        path: path("."),
        json: true,
        threshold: None,
        max_avg_braintax: None,
        top: 10,
    };
    let app = App::new(Box::new(walk), Box::new(scorer), Box::new(renderer), config);

    // Act
    let result = app.run();

    // Assert
    let outcome = result.unwrap();
    assert!(outcome.passes);
    assert_eq!(outcome.report.overall.max_cyclomatic, 1);
}

#[test]
fn run_hands_back_what_the_renderer_made_of_the_report() {
    // Arrange
    let walk = TestWalk {
        files: vec![(path("/work/core/src/lib.rs"), "fn trivial() {}".to_string())],
    };
    let config = Config {
        path: path("/work/core"),
        json: false,
        threshold: None,
        max_avg_braintax: None,
        top: 10,
    };
    let app = App::new(
        Box::new(walk),
        Box::new(DefaultScorer::new()),
        Box::new(StubReportRenderer::new()),
        config,
    );

    // Act
    let outcome = app.run().unwrap();

    // Assert
    assert_eq!(outcome.output, "core: 1 functions");
    assert_eq!(outcome.report.version, env!("CARGO_PKG_VERSION"));
}

#[test]
fn run_names_the_report_after_the_last_component_of_its_root() {
    // Arrange
    let walk = TestWalk {
        files: vec![(path("./core/./src/lib.rs"), "fn trivial() {}".to_string())],
    };
    let config = Config {
        path: path("./core/."),
        json: false,
        threshold: None,
        max_avg_braintax: None,
        top: 10,
    };
    let app = App::new(
        Box::new(walk),
        Box::new(DefaultScorer::new()),
        Box::new(StubReportRenderer::new()),
        config,
    );

    // Act
    let outcome = app.run().unwrap();

    // Assert
    assert_eq!(outcome.report.target, "core");
    assert_eq!(outcome.report.functions[0].module, ".");
}

#[test]
fn run_with_a_file_that_cannot_be_read_names_it() {
    // Arrange
    let tree = InMemorySourceTree::new().with_unreadable_file("/work/src/lib.rs");
    let config = Config {
        path: path("/work"),
        json: false,
        threshold: None,
        max_avg_braintax: None,
        top: 10,
    };
    let app = App::new(
        Box::new(SourceWalk::new(Box::new(tree), path("/work"))),
        Box::new(DefaultScorer::new()),
        Box::new(StubReportRenderer::new()),
        config,
    );

    // Act
    let failure = app.run().unwrap_err();

    // Assert
    assert_eq!(failure.to_string(), "failed to read /work/src/lib.rs");
}

#[test]
fn run_with_a_renderer_that_fails_returns_its_failure() {
    // Arrange
    let walk = TestWalk {
        files: vec![(path("src/lib.rs"), "fn trivial() {}".to_string())],
    };
    let config = Config {
        path: path("."),
        json: false,
        threshold: None,
        max_avg_braintax: None,
        top: 10,
    };
    let app = App::new(
        Box::new(walk),
        Box::new(DefaultScorer::new()),
        Box::new(StubReportRenderer::failing()),
        config,
    );

    // Act
    let failure = app.run().unwrap_err();

    // Assert
    assert_eq!(failure.to_string(), "failed to render the report");
}

#[test]
fn run_with_a_threshold_set_still_writes_the_report() {
    // Arrange
    let walk = TestWalk {
        files: vec![(path("src/lib.rs"), "fn trivial() {}".to_string())],
    };
    let renderer = StubReportRenderer::new();
    let config = Config {
        path: path("."),
        json: false,
        threshold: Some(1),
        max_avg_braintax: None,
        top: 10,
    };
    let app = App::new(
        Box::new(walk),
        Box::new(DefaultScorer::new()),
        Box::new(renderer),
        config,
    );

    // Act
    let result = app.run();

    // Assert
    let outcome = result.unwrap();
    assert!(outcome.passes);
    assert!(!outcome.output.is_empty());
}

#[test]
fn run_with_avg_braintax_above_the_limit_returns_failure() {
    // Arrange
    let walk = TestWalk {
        files: vec![(path("src/lib.rs"), "fn trivial() {}".to_string())],
    };
    let config = Config {
        path: path("."),
        json: false,
        threshold: None,
        max_avg_braintax: Some(0.0),
        top: 10,
    };
    let app = App::new(
        Box::new(walk),
        Box::new(DefaultScorer::new()),
        Box::new(StubReportRenderer::new()),
        config,
    );

    // Act
    let result = app.run();

    // Assert
    assert!(!result.unwrap().passes);
}

#[test]
fn run_with_avg_braintax_within_the_limit_returns_success() {
    // Arrange
    let walk = TestWalk {
        files: vec![(path("src/lib.rs"), "fn trivial() {}".to_string())],
    };
    let config = Config {
        path: path("."),
        json: false,
        threshold: None,
        max_avg_braintax: Some(100.0),
        top: 10,
    };
    let app = App::new(
        Box::new(walk),
        Box::new(DefaultScorer::new()),
        Box::new(StubReportRenderer::new()),
        config,
    );

    // Act
    let result = app.run();

    // Assert
    assert!(result.unwrap().passes);
}

#[test]
fn run_with_no_files_names_the_root_it_searched() {
    // Arrange
    let config = Config {
        path: path("/work/empty"),
        json: false,
        threshold: None,
        max_avg_braintax: None,
        top: 10,
    };
    let app = App::new(
        Box::new(NoFilesWalk),
        Box::new(DefaultScorer::new()),
        Box::new(StubReportRenderer::new()),
        config,
    );

    // Act
    let failure = app.run().unwrap_err();

    // Assert
    assert_eq!(
        failure.to_string(),
        "no Rust source files found in /work/empty"
    );
}
