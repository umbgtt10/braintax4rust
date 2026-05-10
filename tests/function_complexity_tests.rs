// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::function_complexity::FunctionComplexity;

#[test]
fn function_complexity_serializes_to_json() {
    // Arrange
    let fc = FunctionComplexity {
        name: "foo".to_string(),
        file: "src/lib.rs".to_string(),
        module: "lib".to_string(),
        cyclomatic: 5,
    };

    // Act
    let json = serde_json::to_string(&fc).unwrap();

    // Assert
    assert!(json.contains("\"cyclomatic\":5"));
}

#[test]
fn function_complexity_deserializes_from_json() {
    // Arrange
    let json = r#"{"name":"bar","file":"src/main.rs","module":"","cyclomatic":3}"#;

    // Act
    let fc: FunctionComplexity = serde_json::from_str(json).unwrap();

    // Assert
    assert_eq!(fc.name, "bar");
    assert_eq!(fc.cyclomatic, 3);
}
