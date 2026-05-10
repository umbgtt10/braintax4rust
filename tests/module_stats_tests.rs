// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::module_stats::ModuleStats;

#[test]
fn module_stats_serializes_to_json() {
    // Arrange
    let ms = ModuleStats {
        path: "src".to_string(),
        function_count: 3,
        avg_cyclomatic: 4.5,
        max_cyclomatic: 10,
        total_cyclomatic: 15,
    };

    // Act
    let json = serde_json::to_string(&ms).unwrap();

    // Assert
    assert!(json.contains("\"avg_cyclomatic\":4.5"));
    assert!(json.contains("\"max_cyclomatic\":10"));
}
