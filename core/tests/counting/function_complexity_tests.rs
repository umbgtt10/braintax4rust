// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::counting::function_complexity::FunctionComplexity;
use serde_json::from_str;
use serde_json::to_string;

#[test]
fn function_complexity_deserializes_from_json() {
    // Arrange
    let json = r#"{"name":"bar","file":"src/main.rs","module":"","cyclomatic":3,"cfg_gates":0,"hidden_deps":0,"hidden_dep_weight":0.0,"hidden_dep_labels":[],"depth":1,"trait_factor":1.0,"braintax":3.0,"braintax_normalized":80}"#;

    // Act
    let fc: FunctionComplexity = from_str(json).unwrap();

    // Assert
    assert_eq!(fc.name, "bar");
    assert_eq!(fc.cyclomatic, 3);
}

#[test]
fn function_complexity_serializes_to_json() {
    // Arrange
    let fc = FunctionComplexity {
        name: "foo".to_string(),
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
    };

    // Act
    let json = to_string(&fc).unwrap();

    // Assert
    assert!(json.contains("\"cyclomatic\":5"));
}
