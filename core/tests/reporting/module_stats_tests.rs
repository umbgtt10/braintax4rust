// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::reporting::module_stats::ModuleStats;
use serde_json::to_string;

#[test]
fn avg_braintax_returns_the_average_it_was_given() {
    // Arrange
    let stats = ModuleStats::default()
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
    let stats = ModuleStats::default()
        .with_avg_braintax(2.5)
        .with_max_braintax(7.5);

    // Act
    let maximum = stats.max_braintax();

    // Assert
    assert!((maximum - 7.5).abs() < f64::EPSILON);
}

#[test]
fn module_stats_serializes_to_json() {
    // Arrange
    let ms = ModuleStats::default()
        .with_path("src".to_string())
        .with_function_count(3)
        .with_avg_cyclomatic(4.5)
        .with_max_cyclomatic(10)
        .with_total_cyclomatic(15)
        .with_avg_braintax(10.0)
        .with_max_braintax(20.0)
        .with_total_braintax(30.0)
        .with_braintax_normalized(33);

    // Act
    let json = to_string(&ms).unwrap();

    // Assert
    assert!(json.contains("\"avg_braintax\":10.0"));
    assert!(json.contains("\"max_braintax\":20.0"));
}
