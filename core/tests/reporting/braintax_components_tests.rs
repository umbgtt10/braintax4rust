// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::reporting::braintax_components::BraintaxComponents;

fn neutral_components() -> BraintaxComponents {
    BraintaxComponents {
        cfg_gates: 0,
        cyclomatic: 1,
        hidden_dep_weight: 0.0,
        depth: 1,
        trait_factor: 1.0,
        name_opacity: 0,
        macro_density: 0,
        generics: 0,
        self_ref_cost: 0.0,
        return_complexity: 0.0,
    }
}

#[test]
fn compute_braintax_adds_fifteen_percent_for_each_level_below_the_first() {
    // Arrange
    let components = BraintaxComponents {
        depth: 3,
        ..neutral_components()
    };

    // Act
    let braintax = components.compute_braintax();

    // Assert
    assert!((braintax - 1.3).abs() < 1e-9);
}

#[test]
fn compute_braintax_adds_the_remaining_components_without_scaling_them() {
    // Arrange
    let components = BraintaxComponents {
        hidden_dep_weight: 0.5,
        name_opacity: 2,
        macro_density: 1,
        generics: 3,
        self_ref_cost: 0.25,
        return_complexity: 0.75,
        ..neutral_components()
    };

    // Act
    let braintax = components.compute_braintax();

    // Assert
    assert!((braintax - 8.5).abs() < 1e-9);
}

#[test]
fn compute_braintax_doubles_once_per_cfg_gate() {
    // Arrange
    let components = BraintaxComponents {
        cfg_gates: 2,
        ..neutral_components()
    };

    // Act
    let braintax = components.compute_braintax();

    // Assert
    assert!((braintax - 4.0).abs() < f64::EPSILON);
}

#[test]
fn compute_braintax_with_neutral_components_returns_the_cyclomatic_count() {
    // Arrange
    let components = neutral_components();

    // Act
    let braintax = components.compute_braintax();

    // Assert
    assert!((braintax - 1.0).abs() < f64::EPSILON);
}
