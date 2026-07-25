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

fn hidden_deps_of(code: &str) -> HiddenDepsCounter {
    let block: syn::Block = syn::parse_str(code).unwrap();
    let mut counter = HiddenDepsCounter::new();
    counter.visit_block(&block);
    counter
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

#[test]
fn unsafe_block_weight_is_8() {
    // Arrange & Act
    let counter = hidden_deps_of("{ unsafe { let p = std::ptr::null(); } }");

    // Assert
    assert_eq!(counter.weight, 8.0);
}

#[test]
fn println_weight_is_2() {
    // Arrange & Act
    let counter = hidden_deps_of("{ println!(\"hello\"); }");

    // Assert
    assert_eq!(counter.weight, 2.0);
}

#[test]
fn multiple_hidden_deps_sum_weight() {
    // Arrange & Act
    let counter = hidden_deps_of("{ unsafe { }  println!(\"a\");  Instant::now(); }");

    // Assert
    assert_eq!(counter.weight, 8.0 + 2.0 + 4.0);
}

#[test]
fn unsafe_block_label_is_captured() {
    // Arrange & Act
    let counter = hidden_deps_of("{ unsafe { let p = std::ptr::null(); } }");

    // Assert
    assert_eq!(counter.labels, vec!["unsafe".to_string()]);
}

#[test]
fn multiple_hidden_deps_labels_captured_in_order() {
    // Arrange & Act
    let counter = hidden_deps_of("{ unsafe { }  println!(\"a\");  Instant::now(); }");

    // Assert
    assert_eq!(
        counter.labels,
        vec![
            "unsafe".to_string(),
            "println".to_string(),
            "Instant::now".to_string(),
        ]
    );
}

#[test]
fn fully_qualified_instant_now_counts_1() {
    // Arrange & Act
    let count = count_hidden("{ let _ = std::time::Instant::now(); }");

    // Assert
    assert_eq!(count, 1);
}

#[test]
fn fully_qualified_instant_now_label_is_canonical_short_form() {
    // Arrange & Act
    let counter = hidden_deps_of("{ let _ = std::time::Instant::now(); }");

    // Assert
    assert_eq!(counter.labels, vec!["Instant::now".to_string()]);
}

#[test]
fn third_party_qualified_fs_read_does_not_count() {
    // Arrange & Act
    let count = count_hidden("{ let _ = mycrate::fs::read(\"x\"); }");

    // Assert
    assert_eq!(count, 0);
}

#[test]
fn bare_fs_read_counts_1() {
    // Arrange & Act
    let count = count_hidden("{ let _ = fs::read(\"x\"); }");

    // Assert
    assert_eq!(count, 1);
}

#[test]
fn rand_random_qualified_counts_1() {
    // Arrange & Act
    let count = count_hidden("{ let _ = rand::random(); }");

    // Assert
    assert_eq!(count, 1);
}

#[test]
fn rand_thread_rng_qualified_counts_1() {
    // Arrange & Act
    let count = count_hidden("{ let _ = rand::thread_rng(); }");

    // Assert
    assert_eq!(count, 1);
}

#[test]
fn process_abort_qualified_counts_1() {
    // Arrange & Act
    let count = count_hidden("{ std::process::abort(); }");

    // Assert
    assert_eq!(count, 1);
}
