// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::braintax_report::BraintaxReport;
use braintax::function_complexity::FunctionComplexity;
use braintax::module_stats::ModuleStats;
use braintax::overall_stats::OverallStats;

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
        },
        modules: vec![ModuleStats {
            path: "src".to_string(),
            function_count: 2,
            avg_cyclomatic: 3.5,
            max_cyclomatic: 5,
            total_cyclomatic: 7,
            avg_braintax: 4.5,
            max_braintax: 6.0,
        }],
        functions: vec![
            FunctionComplexity {
                name: "foo".to_string(),
                file: "src/lib.rs".to_string(),
                module: "src".to_string(),
                cyclomatic: 2,
                cfg_gates: 0,
                hidden_deps: 0,
                depth: 1,
                trait_factor: 1.0,
                braintax: 2.0,
            },
            FunctionComplexity {
                name: "bar".to_string(),
                file: "src/lib.rs".to_string(),
                module: "src".to_string(),
                cyclomatic: 5,
                cfg_gates: 0,
                hidden_deps: 0,
                depth: 1,
                trait_factor: 1.0,
                braintax: 5.0,
            },
        ],
    };

    // Act
    let json = serde_json::to_string_pretty(&report).unwrap();

    // Assert
    assert!(json.contains("\"version\": \"0.2.0\""));
    assert!(json.contains("\"cyclomatic\": 5"));
    assert!(json.contains("\"avg_braintax\": 4.5"));
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
            "max_braintax": 6.0
        },
        "modules": [
            {
                "path": "src",
                "function_count": 2,
                "avg_cyclomatic": 3.5,
                "max_cyclomatic": 5,
                "total_cyclomatic": 7,
                "avg_braintax": 4.5,
                "max_braintax": 6.0
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
                "depth": 1,
                "trait_factor": 1.0,
                "braintax": 2.0
            }
        ]
    }"#;

    // Act
    let report: BraintaxReport = serde_json::from_str(json).unwrap();

    // Assert
    assert_eq!(report.overall.total_functions, 2);
    assert_eq!(report.functions[0].cyclomatic, 2);
}
