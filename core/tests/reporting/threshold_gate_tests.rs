// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::reporting::overall_stats::OverallStats;
use braintax::reporting::threshold_gate::ThresholdGate;

fn overall(max_cyclomatic: u32, avg_braintax: f64) -> OverallStats {
    OverallStats {
        total_functions: 1,
        avg_cyclomatic: max_cyclomatic.into(),
        max_cyclomatic,
        total_cyclomatic: max_cyclomatic,
        avg_braintax,
        max_braintax: avg_braintax,
        total_braintax: avg_braintax,
        braintax_normalized: 0,
    }
}

#[test]
fn passes_with_avg_braintax_above_limit_returns_false() {
    // Arrange
    let gate = ThresholdGate::new(None, Some(9.25));

    // Act
    let passed = gate.passes(&overall(0, 9.26));

    // Assert
    assert!(!passed);
}

#[test]
fn passes_with_avg_braintax_below_limit_returns_true() {
    // Arrange
    let gate = ThresholdGate::new(None, Some(9.25));

    // Act
    let passed = gate.passes(&overall(0, 9.24));

    // Assert
    assert!(passed);
}

#[test]
fn passes_with_avg_braintax_exactly_at_limit_returns_true() {
    // Arrange
    let gate = ThresholdGate::new(None, Some(9.25));

    // Act
    let passed = gate.passes(&overall(0, 9.25));

    // Assert
    assert!(passed);
}

#[test]
fn passes_with_avg_braintax_limit_set_ignores_a_high_cyclomatic_returns_true() {
    // Arrange
    let gate = ThresholdGate::new(None, Some(9.25));

    // Act
    let passed = gate.passes(&overall(500, 1.0));

    // Assert
    assert!(passed);
}

#[test]
fn passes_with_both_limits_met_returns_true() {
    // Arrange
    let gate = ThresholdGate::new(Some(10), Some(9.25));

    // Act
    let passed = gate.passes(&overall(10, 9.25));

    // Assert
    assert!(passed);
}

#[test]
fn passes_with_cyclomatic_above_limit_returns_false() {
    // Arrange
    let gate = ThresholdGate::new(Some(10), None);

    // Act
    let passed = gate.passes(&overall(11, 0.0));

    // Assert
    assert!(!passed);
}

#[test]
fn passes_with_cyclomatic_below_limit_returns_true() {
    // Arrange
    let gate = ThresholdGate::new(Some(10), None);

    // Act
    let passed = gate.passes(&overall(9, 0.0));

    // Assert
    assert!(passed);
}

#[test]
fn passes_with_cyclomatic_exactly_at_limit_returns_true() {
    // Arrange
    let gate = ThresholdGate::new(Some(10), None);

    // Act
    let passed = gate.passes(&overall(10, 0.0));

    // Assert
    assert!(passed);
}

#[test]
fn passes_with_cyclomatic_limit_set_ignores_a_high_avg_braintax_returns_true() {
    // Arrange
    let gate = ThresholdGate::new(Some(10), None);

    // Act
    let passed = gate.passes(&overall(5, 500.0));

    // Assert
    assert!(passed);
}

#[test]
fn passes_with_no_thresholds_set_returns_true() {
    // Arrange
    let gate = ThresholdGate::new(None, None);

    // Act
    let passed = gate.passes(&overall(999, 999.0));

    // Assert
    assert!(passed);
}

#[test]
fn passes_with_only_avg_braintax_exceeded_returns_false() {
    // Arrange
    let gate = ThresholdGate::new(Some(10), Some(9.25));

    // Act
    let passed = gate.passes(&overall(1, 9.26));

    // Assert
    assert!(!passed);
}

#[test]
fn passes_with_only_cyclomatic_exceeded_returns_false() {
    // Arrange
    let gate = ThresholdGate::new(Some(10), Some(9.25));

    // Act
    let passed = gate.passes(&overall(11, 1.0));

    // Assert
    assert!(!passed);
}
