// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::derive_attr_scorer::DeriveAttrScorer;
use syn::parse_quote;

#[test]
fn known_derive_scores_zero() {
    // Arrange
    let scorer = DeriveAttrScorer::new();
    let attr: syn::Attribute = parse_quote!(#[derive(Debug)]);

    // Act
    let score = scorer.score(&attr);

    // Assert
    assert_eq!(score, 0);
}

#[test]
fn unknown_derive_scores_3() {
    // Arrange
    let scorer = DeriveAttrScorer::new();
    let attr: syn::Attribute = parse_quote!(#[derive(MyCustomDerive)]);

    // Act
    let score = scorer.score(&attr);

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn mixed_derives_sum_with_unknown() {
    // Arrange
    let scorer = DeriveAttrScorer::new();
    let attr: syn::Attribute = parse_quote!(#[derive(Debug, Clone, MyTrait, Eq)]);

    // Act
    let score = scorer.score(&attr);

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn require_list_fails_returns_zero() {
    // Arrange
    let scorer = DeriveAttrScorer::new();
    let attr: syn::Attribute = parse_quote!(#[test]);

    // Act
    let score = scorer.score(&attr);

    // Assert
    assert_eq!(score, 0);
}

#[test]
fn multiple_unknown_derives_sum_to_6() {
    // Arrange
    let scorer = DeriveAttrScorer::new();
    let attr: syn::Attribute = parse_quote!(#[derive(MyTrait, OtherTrait)]);

    // Act
    let score = scorer.score(&attr);

    // Assert
    assert_eq!(score, 6);
}
