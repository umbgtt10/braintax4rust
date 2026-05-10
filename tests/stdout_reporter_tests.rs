// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::braintax_report::BraintaxReport;
use braintax::function_complexity::FunctionComplexity;
use braintax::module_stats::ModuleStats;
use braintax::overall_stats::OverallStats;
use braintax::stdout_reporter::StdoutReporter;
use braintax::traits::reporter::Reporter;

fn sample_report() -> BraintaxReport {
    BraintaxReport {
        version: "0.2.0".to_string(),
        target: "braintax".to_string(),
        overall: OverallStats {
            total_functions: 2,
            avg_cyclomatic: 3.5,
            max_cyclomatic: 5,
            total_cyclomatic: 7,
        },
        modules: vec![ModuleStats {
            path: "lib".to_string(),
            function_count: 2,
            avg_cyclomatic: 3.5,
            max_cyclomatic: 5,
            total_cyclomatic: 7,
        }],
        functions: vec![
            FunctionComplexity {
                name: "simple".to_string(),
                file: "src/lib.rs".to_string(),
                module: "lib".to_string(),
                cyclomatic: 2,
            },
            FunctionComplexity {
                name: "complex".to_string(),
                file: "src/lib.rs".to_string(),
                module: "lib".to_string(),
                cyclomatic: 5,
            },
        ],
    }
}

#[test]
fn reporter_renders_json_output() {
    // Arrange
    let reporter = StdoutReporter::new(true, 10);
    let report = sample_report();

    // Act
    let rendered = reporter.render(&report).unwrap();

    // Assert
    assert!(rendered.starts_with('{'));
    assert!(rendered.contains("\"version\": \"0.2.0\""));
    assert!(rendered.contains("\"cyclomatic\": 5"));
}

#[test]
fn reporter_renders_human_output() {
    // Arrange
    let reporter = StdoutReporter::new(false, 10);
    let report = sample_report();

    // Act
    let rendered = reporter.render(&report).unwrap();

    // Assert
    assert!(rendered.contains("cargo-braintax4rust"));
    assert!(rendered.contains("Overall cyclomatic complexity"));
    assert!(rendered.contains("braintax"));
    assert!(rendered.contains("Top"));
    assert!(rendered.contains("simple"));
    assert!(rendered.contains("complex"));
}

#[test]
fn reporter_render_error_on_failed_serialization() {
    // Arrange
    let reporter = StdoutReporter::new(true, 10);
    let report = sample_report();

    // Act
    let rendered = reporter.render(&report).unwrap();

    // Assert
    assert!(!rendered.is_empty());
}

#[test]
fn reporter_human_shows_top_functions() {
    // Arrange
    let reporter = StdoutReporter::new(false, 1);
    let report = sample_report();

    // Act
    let rendered = reporter.render(&report).unwrap();

    // Assert
    assert!(rendered.contains("Top 1"));
    assert!(rendered.contains("complex"));
}
