// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::reporting::overall_stats::OverallStats;
use serde_json::to_string;

#[test]
fn avg_braintax_returns_the_average_it_was_given() {
    // Arrange
    let stats = OverallStats::default()
        .with_avg_braintax(2.5)
        .with_max_braintax(7.5);

    // Act
    let average = stats.avg_braintax();

    // Assert
    assert!((average - 2.5).abs() < f64::EPSILON);
}

#[test]
fn max_braintax_returns_the_maximum_it_was_given() {
    // Arrange
    let stats = OverallStats::default()
        .with_avg_braintax(2.5)
        .with_max_braintax(7.5);

    // Act
    let maximum = stats.max_braintax();

    // Assert
    assert!((maximum - 7.5).abs() < f64::EPSILON);
}

#[test]
fn overall_stats_serializes_to_json() {
    // Arrange
    let os = OverallStats::default()
        .with_total_functions(10)
        .with_avg_cyclomatic(3.2)
        .with_max_cyclomatic(8)
        .with_total_cyclomatic(32)
        .with_avg_braintax(8.0)
        .with_max_braintax(16.0)
        .with_total_braintax(80.0)
        .with_braintax_normalized(47);

    // Act
    let json = to_string(&os).unwrap();

    // Assert
    assert!(json.contains("\"total_functions\":10"));
    assert!(json.contains("\"avg_braintax\":8.0"));
}
