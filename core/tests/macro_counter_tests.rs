// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::macro_counter::MacroCounter;
use syn::visit::Visit;

fn score_block(code: &str) -> u32 {
    let block: syn::Block = syn::parse_str(code).unwrap();
    let mut counter = MacroCounter::new();
    counter.visit_block(&block);
    counter.count
}

#[test]
fn empty_block_scores_zero() {
    // Arrange & Act
    let score = score_block("{}");

    // Assert
    assert_eq!(score, 0);
}

#[test]
fn known_std_macro_scores_zero() {
    // Arrange & Act
    let score = score_block("{ let _ = vec![1, 2, 3]; }");

    // Assert
    assert_eq!(score, 0);
}

#[test]
fn known_format_macro_scores_zero() {
    // Arrange & Act
    let score = score_block("{ let _ = format!(\"hello\"); }");

    // Assert
    assert_eq!(score, 0);
}

#[test]
fn known_assert_macro_scores_zero() {
    // Arrange & Act
    let score = score_block("{ assert!(true); }");

    // Assert
    assert_eq!(score, 0);
}

#[test]
fn println_not_counted_by_macro_counter() {
    // Arrange & Act
    let score = score_block("{ println!(\"test\"); }");

    // Assert
    assert_eq!(score, 0);
}

#[test]
fn unknown_custom_macro_in_expr_scores_3() {
    // Arrange & Act
    let score = score_block("{ let _ = custom_add!(1, 2); }");

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn multiple_unknown_macros_sum() {
    // Arrange & Act
    let score = score_block("{ my_macro!(); other_macro!(); }");

    // Assert
    assert_eq!(score, 6);
}

#[test]
fn mixed_known_and_unknown_counts_only_unknown() {
    // Arrange & Act
    let score = score_block("{ let _ = vec![1]; my_macro!(); }");

    // Assert
    assert_eq!(score, 3);
}
