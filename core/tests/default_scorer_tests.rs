// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::braintax_normalizer::BraintaxNormalizer;
use braintax::default_scorer::DefaultScorer;
use braintax::function_complexity::FunctionComplexity;
use braintax::traits::scorer::Scorer;

fn make_fn(name: &str, module: &str, cyclomatic: u32) -> FunctionComplexity {
    let braintax = cyclomatic as f64;
    FunctionComplexity {
        name: name.to_string(),
        file: format!("src/{}.rs", module),
        module: module.to_string(),
        cyclomatic,
        cfg_gates: 0,
        hidden_deps: 0,
        hidden_dep_weight: 0.0,
        hidden_dep_labels: vec![],
        depth: 1,
        trait_factor: 1.0,
        braintax,
        braintax_normalized: BraintaxNormalizer::new().normalize(braintax),
    }
}

#[test]
fn overall_stats_empty_returns_zeros() {
    // Arrange
    let scorer = DefaultScorer::new();
    let functions = vec![];

    // Act
    let stats = scorer.overall_stats(&functions);

    // Assert
    assert_eq!(stats.total_functions, 0);
    assert_eq!(stats.avg_cyclomatic, 0.0);
    assert_eq!(stats.max_cyclomatic, 0);
    assert_eq!(stats.total_cyclomatic, 0);
    assert_eq!(stats.total_braintax, 0.0);
    assert_eq!(stats.braintax_normalized, 0);
}

#[test]
fn overall_stats_with_one_fn() {
    // Arrange
    let scorer = DefaultScorer::new();
    let functions = vec![make_fn("foo", "lib", 5)];

    // Act
    let stats = scorer.overall_stats(&functions);

    // Assert
    assert_eq!(stats.total_functions, 1);
    assert_eq!(stats.avg_cyclomatic, 5.0);
    assert_eq!(stats.max_cyclomatic, 5);
    assert_eq!(stats.total_cyclomatic, 5);
    assert_eq!(stats.total_braintax, 5.0);
    assert_eq!(stats.braintax_normalized, 67);
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
    assert_eq!(stats.total_functions, 3);
    assert_eq!(stats.avg_cyclomatic, (1.0 + 5.0 + 10.0) / 3.0);
    assert_eq!(stats.max_cyclomatic, 10);
    assert_eq!(stats.total_cyclomatic, 16);
    assert_eq!(stats.total_braintax, 16.0);
    assert_eq!(stats.braintax_normalized, 64);
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
    let foo_mod = modules.iter().find(|m| m.path == "foo").unwrap();
    assert_eq!(foo_mod.function_count, 2);
    assert_eq!(foo_mod.avg_cyclomatic, 3.0);
    assert_eq!(foo_mod.max_cyclomatic, 5);
    assert_eq!(foo_mod.total_cyclomatic, 6);
    assert_eq!(foo_mod.total_braintax, 6.0);
    assert_eq!(foo_mod.braintax_normalized, 80);

    let bar_mod = modules.iter().find(|m| m.path == "bar").unwrap();
    assert_eq!(bar_mod.function_count, 1);
    assert_eq!(bar_mod.max_cyclomatic, 10);
    assert_eq!(bar_mod.total_braintax, 10.0);
    assert_eq!(bar_mod.braintax_normalized, 33);
}
