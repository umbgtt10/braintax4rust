// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::overall_stats::OverallStats;

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
    };

    // Act
    let json = serde_json::to_string(&os).unwrap();

    // Assert
    assert!(json.contains("\"total_functions\":10"));
    assert!(json.contains("\"avg_braintax\":8.0"));
}
