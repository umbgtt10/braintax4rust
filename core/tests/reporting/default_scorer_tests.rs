// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::counting::function_complexity::FunctionComplexity;
use braintax::reporting::braintax_normalizer::BraintaxNormalizer;
use braintax::reporting::default_scorer::DefaultScorer;
use braintax::traits::scorer::Scorer;

fn make_fn(name: &str, module: &str, cyclomatic: u32) -> FunctionComplexity {
    let braintax = cyclomatic as f64;
    FunctionComplexity::default()
        .with_name(name.to_string())
        .with_file(format!("src/{}.rs", module))
        .with_module(module.to_string())
        .with_cyclomatic(cyclomatic)
        .with_depth(1)
        .with_trait_factor(1.0)
        .with_braintax(braintax)
        .with_braintax_normalized(BraintaxNormalizer::new().normalize(braintax))
}

// Only the structural components reach a function's score; the rest are zero.
#[test]
fn compute_braintax_of_a_function_scales_its_cyclomatic_count_by_cfg_gates_and_depth() {
    // Arrange
    let function = make_fn("f", "m", 4).with_cfg_gates(1).with_depth(3);

    // Act
    let braintax = DefaultScorer::compute_braintax(&function);

    // Assert
    assert!((braintax - 10.4).abs() < 1e-9, "got {braintax}");
}

#[test]
fn module_stats_groups_by_module() {
    // Arrange
    let scorer = DefaultScorer::new();
    let functions = vec![
        make_fn("a", "foo", 1),
        make_fn("b", "foo", 5),
        make_fn("c", "bar", 10),
    ];

    // Act
    let modules = scorer.module_stats(&functions);

    // Assert
    assert_eq!(modules.len(), 2);
    let foo_mod = modules.iter().find(|m| m.path() == "foo").unwrap();
    assert_eq!(foo_mod.function_count(), 2);
    assert_eq!(foo_mod.avg_cyclomatic(), 3.0);
    assert_eq!(foo_mod.max_cyclomatic(), 5);
    assert_eq!(foo_mod.total_cyclomatic(), 6);
    assert_eq!(foo_mod.total_braintax(), 6.0);
    assert_eq!(foo_mod.braintax_normalized(), 80);

    let bar_mod = modules.iter().find(|m| m.path() == "bar").unwrap();
    assert_eq!(bar_mod.function_count(), 1);
    assert_eq!(bar_mod.max_cyclomatic(), 10);
    assert_eq!(bar_mod.total_braintax(), 10.0);
    assert_eq!(bar_mod.braintax_normalized(), 33);
}

#[test]
fn overall_stats_empty_returns_zeros() {
    // Arrange
    let scorer = DefaultScorer::new();
    let functions = vec![];

    // Act
    let stats = scorer.overall_stats(&functions);

    // Assert
    assert_eq!(stats.total_functions(), 0);
    assert_eq!(stats.avg_cyclomatic(), 0.0);
    assert_eq!(stats.max_cyclomatic(), 0);
    assert_eq!(stats.total_cyclomatic(), 0);
    assert_eq!(stats.total_braintax(), 0.0);
    assert_eq!(stats.braintax_normalized(), 0);
}

#[test]
fn overall_stats_with_multiple_fns() {
    // Arrange
    let scorer = DefaultScorer::new();
    let functions = vec![
        make_fn("a", "lib", 1),
        make_fn("b", "lib", 5),
        make_fn("c", "lib", 10),
    ];

    // Act
    let stats = scorer.overall_stats(&functions);

    // Assert
    assert_eq!(stats.total_functions(), 3);
    assert_eq!(stats.avg_cyclomatic(), (1.0 + 5.0 + 10.0) / 3.0);
    assert_eq!(stats.max_cyclomatic(), 10);
    assert_eq!(stats.total_cyclomatic(), 16);
    assert_eq!(stats.total_braintax(), 16.0);
    assert_eq!(stats.braintax_normalized(), 64);
}

#[test]
fn overall_stats_with_one_fn() {
    // Arrange
    let scorer = DefaultScorer::new();
    let functions = vec![make_fn("foo", "lib", 5)];

    // Act
    let stats = scorer.overall_stats(&functions);

    // Assert
    assert_eq!(stats.total_functions(), 1);
    assert_eq!(stats.avg_cyclomatic(), 5.0);
    assert_eq!(stats.max_cyclomatic(), 5);
    assert_eq!(stats.total_cyclomatic(), 5);
    assert_eq!(stats.total_braintax(), 5.0);
    assert_eq!(stats.braintax_normalized(), 67);
}
