// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::analysis::trait_info::TraitInfo;
use braintax::counting::signature_scorer::SignatureScorer;
use syn::ItemFn;
use syn::Signature;
use syn::TypeParamBound;
use syn::parse_str;

fn bound(source: &str) -> TypeParamBound {
    parse_str(source).expect("bound should parse")
}

fn info(methods: u32, assoc_types: u32, supertraits: u32, impl_count: u32) -> TraitInfo {
    TraitInfo {
        methods,
        assoc_types,
        supertraits,
        impl_count,
    }
}

fn signature(source: &str) -> Signature {
    let parsed: ItemFn = parse_str(source).expect("function should parse");
    parsed.sig
}

#[test]
fn is_marker_supertrait_for_a_lifetime_bound_returns_true() {
    // Arrange & Act
    let marker = SignatureScorer::is_marker_supertrait(&bound("'static"));

    // Assert
    assert!(marker);
}

#[test]
fn is_marker_supertrait_for_a_real_trait_returns_false() {
    // Arrange & Act
    let marker = SignatureScorer::is_marker_supertrait(&bound("Iterator"));

    // Assert
    assert!(!marker);
}

// A marker supertrait adds no methods and no conceptual surface, so adding
// `: Send` to satisfy a lint must not inflate the factor.
#[test]
fn is_marker_supertrait_for_an_auto_trait_returns_true() {
    // Arrange
    let auto_traits = ["Send", "Sync", "Unpin", "Sized"];

    // Act & Assert
    for name in auto_traits {
        assert!(SignatureScorer::is_marker_supertrait(&bound(name)));
    }
}

#[test]
fn return_complexity_for_a_plain_type_returns_zero() {
    // Arrange & Act
    let complexity = SignatureScorer::return_complexity(&signature("fn f() -> u32 {}").output);

    // Assert
    assert!(complexity.abs() < f64::EPSILON);
}

#[test]
fn return_complexity_for_impl_trait_costs_more_than_a_trait_object() {
    // Arrange & Act
    let impl_trait = SignatureScorer::return_complexity(
        &signature("fn f() -> impl Iterator<Item = u32> {}").output,
    );
    let trait_object =
        SignatureScorer::return_complexity(&signature("fn f() -> Box<dyn Iterator> {}").output);

    // Assert
    assert!(impl_trait > trait_object);
}

#[test]
fn return_complexity_for_no_return_type_returns_zero() {
    // Arrange & Act
    let complexity = SignatureScorer::return_complexity(&signature("fn f() {}").output);

    // Assert
    assert!(complexity.abs() < f64::EPSILON);
}

#[test]
fn return_complexity_grows_with_generic_arguments() {
    // Arrange & Act
    let plain = SignatureScorer::return_complexity(&signature("fn f() -> Vec<u32> {}").output);
    let nested =
        SignatureScorer::return_complexity(&signature("fn f() -> HashMap<u32, String> {}").output);

    // Assert
    assert!(nested > plain);
}

#[test]
fn self_ref_cost_for_a_free_function_returns_zero() {
    // Arrange & Act
    let cost = SignatureScorer::self_ref_cost(&signature("fn f(a: u32) {}").inputs);

    // Assert
    assert!(cost.abs() < f64::EPSILON);
}

#[test]
fn self_ref_cost_for_a_mutable_receiver_costs_more_than_a_shared_one() {
    // Arrange & Act
    let shared = SignatureScorer::self_ref_cost(&signature("fn f(&self) {}").inputs);
    let mutable = SignatureScorer::self_ref_cost(&signature("fn f(&mut self) {}").inputs);

    // Assert
    assert!((shared - 0.2).abs() < f64::EPSILON);
    assert!((mutable - 0.4).abs() < f64::EPSILON);
}

#[test]
fn trait_factor_beyond_three_methods_adds_a_penalty() {
    // Arrange & Act
    let three = SignatureScorer::trait_factor("MyTrait", &info(3, 0, 0, 1));
    let ten = SignatureScorer::trait_factor("MyTrait", &info(10, 0, 0, 1));

    // Assert
    assert!(ten > three);
}

#[test]
fn trait_factor_dispatch_penalty_is_capped() {
    // Arrange & Act
    let few = SignatureScorer::trait_factor("MyTrait", &info(1, 0, 0, 4));
    let many = SignatureScorer::trait_factor("MyTrait", &info(1, 0, 0, 400));

    // Assert
    assert!(many >= few);
    assert!(many < 2.0);
}

#[test]
fn trait_factor_for_a_known_std_trait_returns_the_discounted_factor() {
    // Arrange & Act
    let factor = SignatureScorer::trait_factor("Clone", &info(1, 0, 0, 1));

    // Assert
    assert!((factor - 0.80).abs() < f64::EPSILON);
}

#[test]
fn trait_factor_for_a_plain_custom_trait_returns_the_base_factor() {
    // Arrange & Act
    let factor = SignatureScorer::trait_factor("MyTrait", &info(1, 0, 0, 1));

    // Assert
    assert!((factor - 0.90).abs() < 1e-9);
}

#[test]
fn trait_factor_with_associated_types_costs_more_than_without() {
    // Arrange & Act
    let plain = SignatureScorer::trait_factor("MyTrait", &info(1, 0, 0, 1));
    let with_assoc = SignatureScorer::trait_factor("MyTrait", &info(1, 1, 0, 1));

    // Assert
    assert!(with_assoc > plain);
}
