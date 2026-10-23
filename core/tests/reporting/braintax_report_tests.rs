// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::counting::function_complexity::FunctionComplexity;
use braintax::reporting::braintax_report::BraintaxReport;
use braintax::reporting::module_stats::ModuleStats;
use braintax::reporting::overall_stats::OverallStats;
use serde_json::from_str;
use serde_json::to_string_pretty;

#[test]
fn modules_returns_the_modules_it_was_given() {
    // Arrange
    let report = BraintaxReport::new(
        String::from("0.13.1"),
        String::from("node"),
        OverallStats::default(),
        vec![ModuleStats::default().with_path(String::from("node::codec"))],
        Vec::new(),
    );

    // Act
    let modules = report.modules();

    // Assert
    assert_eq!(modules.len(), 1);
    assert_eq!(modules[0].path(), "node::codec");
}

#[test]
fn report_deserializes_from_json() {
    // Arrange
    let json = r#"{
        "version": "0.2.0",
        "target": "test",
        "overall": {
            "total_functions": 2,
            "avg_cyclomatic": 3.5,
            "max_cyclomatic": 5,
            "total_cyclomatic": 7,
            "avg_braintax": 4.5,
            "max_braintax": 6.0,
            "total_braintax": 9.0,
            "braintax_normalized": 70
        },
        "modules": [
            {
                "path": "src",
                "function_count": 2,
                "avg_cyclomatic": 3.5,
                "max_cyclomatic": 5,
                "total_cyclomatic": 7,
                "avg_braintax": 4.5,
                "max_braintax": 6.0,
                "total_braintax": 9.0,
                "braintax_normalized": 70
            }
        ],
        "functions": [
            {
                "name": "foo",
                "file": "src/lib.rs",
                "module": "src",
                "cyclomatic": 2,
                "cfg_gates": 0,
                "hidden_deps": 0,
                "hidden_dep_weight": 0.0,
                "hidden_dep_labels": [],
                "depth": 1,
                "trait_factor": 1.0,
                "braintax": 2.0,
                "braintax_normalized": 87
            }
        ]
    }"#;

    // Act
    let report: BraintaxReport = from_str(json).unwrap();

    // Assert
    assert_eq!(report.overall().total_functions(), 2);
    assert_eq!(report.functions()[0].cyclomatic(), 2);
}

#[test]
fn report_serializes_to_json() {
    // Arrange
    let report = BraintaxReport::new(
        "0.2.0".to_string(),
        "test".to_string(),
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
                .with_path("src".to_string())
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
                .with_name("foo".to_string())
                .with_file("src/lib.rs".to_string())
                .with_module("src".to_string())
                .with_cyclomatic(2)
                .with_depth(1)
                .with_trait_factor(1.0)
                .with_braintax(2.0)
                .with_braintax_normalized(87),
            FunctionComplexity::default()
                .with_name("bar".to_string())
                .with_file("src/lib.rs".to_string())
                .with_module("src".to_string())
                .with_cyclomatic(5)
                .with_depth(1)
                .with_trait_factor(1.0)
                .with_braintax(5.0)
                .with_braintax_normalized(67),
        ],
    );

    // Act
    let json = to_string_pretty(&report).unwrap();

    // Assert
    assert!(json.contains("\"version\": \"0.2.0\""));
    assert!(json.contains("\"cyclomatic\": 5"));
    assert!(json.contains("\"avg_braintax\": 4.5"));
}

#[test]
fn target_returns_the_target_it_was_given() {
    // Arrange
    let report = BraintaxReport::new(
        String::from("0.13.1"),
        String::from("node"),
        OverallStats::default(),
        vec![ModuleStats::default().with_path(String::from("node::codec"))],
        Vec::new(),
    );

    // Act
    let target = report.target();

    // Assert
    assert_eq!(target, "node");
}

#[test]
fn version_returns_the_version_it_was_given() {
    // Arrange
    let report = BraintaxReport::new(
        String::from("0.13.1"),
        String::from("node"),
        OverallStats::default(),
        vec![ModuleStats::default().with_path(String::from("node::codec"))],
        Vec::new(),
    );

    // Act
    let version = report.version();

    // Assert
    assert_eq!(version, "0.13.1");
}
