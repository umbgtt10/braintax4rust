// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::name_opacity_counter::NameOpacityCounter;
use syn::visit::Visit;

fn score_block(code: &str) -> u32 {
    let block: syn::Block = syn::parse_str(code).unwrap();
    let mut counter = NameOpacityCounter::new();
    counter.visit_block(&block);
    counter.score
}

fn score_params(code: &str) -> u32 {
    let sig: syn::Signature = syn::parse_str(code).unwrap();
    let mut counter = NameOpacityCounter::new();
    counter.visit_params(&sig.inputs);
    counter.score
}

#[test]
fn empty_block_scores_zero() {
    // Arrange & Act
    let score = score_block("{}");

    // Assert
    assert_eq!(score, 0);
}

#[test]
fn one_char_let_scores_2() {
    // Arrange & Act
    let score = score_block("{ let x = 1; }");

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn two_char_let_scores_1() {
    // Arrange & Act
    let score = score_block("{ let cx = 1; }");

    // Assert
    assert_eq!(score, 1);
}

#[test]
fn four_char_let_scores_0() {
    // Arrange & Act
    let score = score_block("{ let data = 1; }");

    // Assert
    assert_eq!(score, 0);
}

#[test]
fn single_char_param_scores_2() {
    // Arrange & Act
    let score = score_params("fn foo(x: i32)");

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn long_param_scores_0() {
    // Arrange & Act
    let score = score_params("fn foo(value: i32)");

    // Assert
    assert_eq!(score, 0);
}

#[test]
fn for_var_counts_as_pattern() {
    // Arrange & Act
    let score = score_block("{ for i in 0..10 {} }");

    // Assert
    assert_eq!(score, 2);
}

#[test]
fn multiple_identifiers_stack() {
    // Arrange & Act
    let score = score_block("{ let x = 1; let val = 2; let data = 3; }");

    // Assert
    assert_eq!(score, 3); // x=2 + val=1 + data=0
}

#[test]
fn match_arm_binding_counts() {
    // Arrange & Act
    let score = score_block("{ match Some(1) { Some(n) => {}, None => {} } }");

    // Assert
    assert_eq!(score, 2); // n=2, None is not a binding
}
