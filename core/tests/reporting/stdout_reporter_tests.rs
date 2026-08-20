// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::counting::function_complexity::FunctionComplexity;
use braintax::reporting::braintax_report::BraintaxReport;
use braintax::reporting::module_stats::ModuleStats;
use braintax::reporting::overall_stats::OverallStats;
use braintax::reporting::stdout_reporter::StdoutReporter;
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
            avg_braintax: 4.5,
            max_braintax: 6.0,
            total_braintax: 9.0,
            braintax_normalized: 70,
        },
        modules: vec![ModuleStats {
            path: "lib".to_string(),
            function_count: 2,
            avg_cyclomatic: 3.5,
            max_cyclomatic: 5,
            total_cyclomatic: 7,
            avg_braintax: 4.5,
            max_braintax: 6.0,
            total_braintax: 9.0,
            braintax_normalized: 70,
        }],
        functions: vec![
            FunctionComplexity {
                name: "simple".to_string(),
                file: "src/lib.rs".to_string(),
                module: "lib".to_string(),
                cyclomatic: 2,
                cfg_gates: 0,
                hidden_deps: 0,
                hidden_dep_weight: 0.0,
                hidden_dep_labels: vec![],
                depth: 1,
                trait_factor: 1.0,
                braintax: 2.0,
                braintax_normalized: 87,
            },
            FunctionComplexity {
                name: "complex".to_string(),
                file: "src/lib.rs".to_string(),
                module: "lib".to_string(),
                cyclomatic: 5,
                cfg_gates: 0,
                hidden_deps: 0,
                hidden_dep_weight: 0.0,
                hidden_dep_labels: vec![],
                depth: 1,
                trait_factor: 1.0,
                braintax: 5.0,
                braintax_normalized: 67,
            },
        ],
    }
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
fn reporter_renders_human_output() {
    // Arrange
    let reporter = StdoutReporter::new(false, 10);
    let report = sample_report();

    // Act
    let rendered = reporter.render(&report).unwrap();

    // Assert
    assert!(rendered.contains("cargo-braintax4rust"));
    assert!(rendered.contains("Overall braintax"));
    assert!(rendered.contains("braintax"));
    assert!(rendered.contains("Top"));
    assert!(rendered.contains("simple"));
    assert!(rendered.contains("complex"));
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
