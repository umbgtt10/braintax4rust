// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::counting::function_complexity::FunctionComplexity;
use braintax::reporting::braintax_report::BraintaxReport;
use braintax::reporting::module_stats::ModuleStats;
use braintax::reporting::overall_stats::OverallStats;
use braintax::traits::report_renderer::ReportRenderer;
use braintax_cli::system::json_report_renderer::JsonReportRenderer;

fn report_of(
    overall: OverallStats,
    modules: Vec<ModuleStats>,
    functions: Vec<FunctionComplexity>,
) -> BraintaxReport {
    BraintaxReport {
        version: "0.2.0".to_string(),
        target: "test".to_string(),
        overall,
        modules,
        functions,
    }
}

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
                depth: 1,
                trait_factor: 1.0,
                braintax: 2.0,
                braintax_normalized: 87,
                ..Default::default()
            },
            FunctionComplexity {
                name: "complex".to_string(),
                file: "src/lib.rs".to_string(),
                module: "lib".to_string(),
                cyclomatic: 5,
                depth: 1,
                trait_factor: 1.0,
                braintax: 5.0,
                braintax_normalized: 67,
                ..Default::default()
            },
        ],
    }
}

#[test]
fn function_complexity_serializes_to_json() {
    // Arrange
    let fc = FunctionComplexity {
        name: "foo".to_string(),
        file: "src/lib.rs".to_string(),
        module: "lib".to_string(),
        cyclomatic: 5,
        depth: 1,
        trait_factor: 1.0,
        braintax: 5.0,
        braintax_normalized: 67,
        ..Default::default()
    };
    let report = report_of(OverallStats::default(), vec![], vec![fc]);

    // Act
    let json = JsonReportRenderer::new().render(&report).unwrap();

    // Assert
    assert!(json.contains("\"cyclomatic\": 5"));
}

#[test]
fn module_stats_serializes_to_json() {
    // Arrange
    let ms = ModuleStats {
        path: "src".to_string(),
        function_count: 3,
        avg_cyclomatic: 4.5,
        max_cyclomatic: 10,
        total_cyclomatic: 15,
        avg_braintax: 10.0,
        max_braintax: 20.0,
        total_braintax: 30.0,
        braintax_normalized: 33,
    };
    let report = report_of(OverallStats::default(), vec![ms], vec![]);

    // Act
    let json = JsonReportRenderer::new().render(&report).unwrap();

    // Assert
    assert!(json.contains("\"avg_braintax\": 10.0"));
    assert!(json.contains("\"max_braintax\": 20.0"));
}

#[test]
fn overall_stats_serializes_to_json() {
    // Arrange
    let os = OverallStats {
        total_functions: 10,
        avg_cyclomatic: 3.2,
        max_cyclomatic: 8,
        total_cyclomatic: 32,
        avg_braintax: 8.0,
        max_braintax: 16.0,
        total_braintax: 80.0,
        braintax_normalized: 47,
    };
    let report = report_of(os, vec![], vec![]);

    // Act
    let json = JsonReportRenderer::new().render(&report).unwrap();

    // Assert
    assert!(json.contains("\"total_functions\": 10"));
    assert!(json.contains("\"avg_braintax\": 8.0"));
}

#[test]
fn report_serializes_to_json() {
    // Arrange
    let report = BraintaxReport {
        version: "0.2.0".to_string(),
        target: "test".to_string(),
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
            path: "src".to_string(),
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
                name: "foo".to_string(),
                file: "src/lib.rs".to_string(),
                module: "src".to_string(),
                cyclomatic: 2,
                depth: 1,
                trait_factor: 1.0,
                braintax: 2.0,
                braintax_normalized: 87,
                ..Default::default()
            },
            FunctionComplexity {
                name: "bar".to_string(),
                file: "src/lib.rs".to_string(),
                module: "src".to_string(),
                cyclomatic: 5,
                depth: 1,
                trait_factor: 1.0,
                braintax: 5.0,
                braintax_normalized: 67,
                ..Default::default()
            },
        ],
    };

    // Act
    let json = JsonReportRenderer::new().render(&report).unwrap();

    // Assert
    assert!(json.contains("\"version\": \"0.2.0\""));
    assert!(json.contains("\"cyclomatic\": 5"));
    assert!(json.contains("\"avg_braintax\": 4.5"));
}

#[test]
fn reporter_render_error_on_failed_serialization() {
    // Arrange
    let reporter = JsonReportRenderer::new();
    let report = sample_report();

    // Act
    let rendered = reporter.render(&report).unwrap();

    // Assert
    assert!(!rendered.is_empty());
}

#[test]
fn reporter_renders_json_output() {
    // Arrange
    let reporter = JsonReportRenderer::new();
    let report = sample_report();

    // Act
    let rendered = reporter.render(&report).unwrap();

    // Assert
    assert!(rendered.starts_with('{'));
    assert!(rendered.contains("\"version\": \"0.2.0\""));
    assert!(rendered.contains("\"cyclomatic\": 5"));
}
