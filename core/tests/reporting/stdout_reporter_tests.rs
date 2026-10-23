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
    BraintaxReport::new(
        "0.2.0".to_string(),
        "braintax".to_string(),
        OverallStats::default()
            .with_total_functions(2)
            .with_avg_cyclomatic(3.5)
            .with_max_cyclomatic(5)
            .with_total_cyclomatic(7)
            .with_avg_braintax(4.5)
            .with_max_braintax(6.0)
            .with_total_braintax(9.0)
            .with_braintax_normalized(70),
        vec![
            ModuleStats::default()
                .with_path("lib".to_string())
                .with_function_count(2)
                .with_avg_cyclomatic(3.5)
                .with_max_cyclomatic(5)
                .with_total_cyclomatic(7)
                .with_avg_braintax(4.5)
                .with_max_braintax(6.0)
                .with_total_braintax(9.0)
                .with_braintax_normalized(70),
        ],
        vec![
            FunctionComplexity::default()
                .with_name("simple".to_string())
                .with_file("src/lib.rs".to_string())
                .with_module("lib".to_string())
                .with_cyclomatic(2)
                .with_depth(1)
                .with_trait_factor(1.0)
                .with_braintax(2.0)
                .with_braintax_normalized(87),
            FunctionComplexity::default()
                .with_name("complex".to_string())
                .with_file("src/lib.rs".to_string())
                .with_module("lib".to_string())
                .with_cyclomatic(5)
                .with_depth(1)
                .with_trait_factor(1.0)
                .with_braintax(5.0)
                .with_braintax_normalized(67),
        ],
    )
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
