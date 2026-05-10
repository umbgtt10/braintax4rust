// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::hidden_deps_counter::HiddenDepsCounter;
use syn::visit::Visit;

fn count_hidden(code: &str) -> u32 {
    let block: syn::Block = syn::parse_str(code).unwrap();
    let mut counter = HiddenDepsCounter::new();
    counter.visit_block(&block);
    counter.count
}

#[test]
fn empty_block_has_zero_hidden_deps() {
    // Arrange & Act
    let count = count_hidden("{}");

    // Assert
    assert_eq!(count, 0);
}

#[test]
fn unsafe_block_counts_1() {
    // Arrange & Act
    let count = count_hidden("{ unsafe { let p = std::ptr::null(); } }");

    // Assert
    assert_eq!(count, 1);
}

#[test]
fn println_counts_1() {
    // Arrange & Act
    let count = count_hidden("{ println!(\"hello\"); }");

    // Assert
    assert_eq!(count, 1);
}

#[test]
fn eprintln_counts_1() {
    // Arrange & Act
    let count = count_hidden("{ eprintln!(\"error\"); }");

    // Assert
    assert_eq!(count, 1);
}

#[test]
fn instant_now_counts_1() {
    // Arrange & Act
    let count = count_hidden("{ let _ = Instant::now(); }");

    // Assert
    assert_eq!(count, 1);
}

#[test]
fn random_counts_1() {
    // Arrange & Act
    let count = count_hidden("{ let _ = random(); }");

    // Assert
    assert_eq!(count, 1);
}

#[test]
fn multiple_hidden_deps_stack() {
    // Arrange & Act
    let count = count_hidden("{ unsafe { }  println!(\"a\");  Instant::now(); }");

    // Assert
    assert_eq!(count, 3);
}
