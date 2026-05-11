// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::generics_counter::GenericsCounter;

fn score(sig: &str) -> u32 {
    let sig: syn::Signature = syn::parse_str(sig).unwrap();
    GenericsCounter::score_generics(&sig.generics.params, &sig.generics.where_clause)
}

#[test]
fn no_generics_scores_zero() {
    // Arrange & Act
    let result = score("fn foo()");

    // Assert
    assert_eq!(result, 0);
}

#[test]
fn single_unconstrained_type_param_scores_2() {
    // Arrange & Act
    let result = score("fn foo<T>()");

    // Assert
    assert_eq!(result, 2);
}

#[test]
fn single_bounded_type_param_scores_3() {
    // Arrange & Act
    let result = score("fn foo<T: Debug>()");

    // Assert
    assert_eq!(result, 3);
}

#[test]
fn multiple_bounds_sum() {
    // Arrange & Act
    let result = score("fn foo<T: Debug + Clone + Eq>()");

    // Assert
    assert_eq!(result, 5);
}

#[test]
fn multiple_type_params_sum() {
    // Arrange & Act
    let result = score("fn foo<A, B>()");

    // Assert
    assert_eq!(result, 4);
}

#[test]
fn where_clause_bounds_counted() {
    // Arrange & Act
    let result = score("fn foo<A>() where A: Debug + Clone");

    // Assert
    assert_eq!(result, 4);
}

#[test]
fn lifetimes_scores_zero() {
    // Arrange & Act
    let result = score("fn foo<'a>(x: &'a str)");

    // Assert
    assert_eq!(result, 0);
}

#[test]
fn const_generic_scores_3() {
    // Arrange & Act
    let result = score("fn foo<const N: usize>()");

    // Assert
    assert_eq!(result, 3);
}

#[test]
fn mixed_generics_sum_correctly() {
    // Arrange & Act
    let result = score("fn foo<A: Clone, B, const N: usize>() where A: Debug");

    // Assert
    assert_eq!(result, 9);
}
