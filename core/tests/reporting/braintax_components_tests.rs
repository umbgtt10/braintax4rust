// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::reporting::braintax_components::BraintaxComponents;

fn neutral_components() -> BraintaxComponents {
    BraintaxComponents::default()
        .with_cyclomatic(1)
        .with_depth(1)
        .with_trait_factor(1.0)
}

#[test]
fn compute_braintax_adds_fifteen_percent_for_each_level_below_the_first() {
    // Arrange
    let components = neutral_components().with_depth(3);

    // Act
    let braintax = components.compute_braintax();

    // Assert
    assert!((braintax - 1.3).abs() < 1e-9);
}

#[test]
fn compute_braintax_adds_the_remaining_components_without_scaling_them() {
    // Arrange
    let components = neutral_components()
        .with_hidden_dep_weight(0.5)
        .with_name_opacity(2)
        .with_macro_density(1)
        .with_generics(3)
        .with_self_ref_cost(0.25)
        .with_return_complexity(0.75);

    // Act
    let braintax = components.compute_braintax();

    // Assert
    assert!((braintax - 8.5).abs() < 1e-9);
}

#[test]
fn compute_braintax_doubles_once_per_cfg_gate() {
    // Arrange
    let components = neutral_components().with_cfg_gates(2);

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

#[test]
fn depth_returns_the_value_it_was_given() {
    // Arrange
    let components = BraintaxComponents::default()
        .with_depth(2)
        .with_name_opacity(3)
        .with_macro_density(4)
        .with_generics(5)
        .with_self_ref_cost(0.5)
        .with_return_complexity(1.5);

    // Act
    let got = components.depth();

    // Assert
    assert_eq!(got, 2);
}

#[test]
fn generics_returns_the_value_it_was_given() {
    // Arrange
    let components = BraintaxComponents::default()
        .with_depth(2)
        .with_name_opacity(3)
        .with_macro_density(4)
        .with_generics(5)
        .with_self_ref_cost(0.5)
        .with_return_complexity(1.5);

    // Act
    let got = components.generics();

    // Assert
    assert_eq!(got, 5);
}

#[test]
fn macro_density_returns_the_value_it_was_given() {
    // Arrange
    let components = BraintaxComponents::default()
        .with_depth(2)
        .with_name_opacity(3)
        .with_macro_density(4)
        .with_generics(5)
        .with_self_ref_cost(0.5)
        .with_return_complexity(1.5);

    // Act
    let got = components.macro_density();

    // Assert
    assert_eq!(got, 4);
}

#[test]
fn name_opacity_returns_the_value_it_was_given() {
    // Arrange
    let components = BraintaxComponents::default()
        .with_depth(2)
        .with_name_opacity(3)
        .with_macro_density(4)
        .with_generics(5)
        .with_self_ref_cost(0.5)
        .with_return_complexity(1.5);

    // Act
    let got = components.name_opacity();

    // Assert
    assert_eq!(got, 3);
}

#[test]
fn return_complexity_returns_the_value_it_was_given() {
    // Arrange
    let components = BraintaxComponents::default()
        .with_depth(2)
        .with_name_opacity(3)
        .with_macro_density(4)
        .with_generics(5)
        .with_self_ref_cost(0.5)
        .with_return_complexity(1.5);

    // Act
    let got = components.return_complexity();

    // Assert
    assert!((got - 1.5).abs() < f64::EPSILON);
}

#[test]
fn self_ref_cost_returns_the_value_it_was_given() {
    // Arrange
    let components = BraintaxComponents::default()
        .with_depth(2)
        .with_name_opacity(3)
        .with_macro_density(4)
        .with_generics(5)
        .with_self_ref_cost(0.5)
        .with_return_complexity(1.5);

    // Act
    let got = components.self_ref_cost();

    // Assert
    assert!((got - 0.5).abs() < f64::EPSILON);
}
