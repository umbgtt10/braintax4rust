// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::complexity_visitor::ComplexityVisitor;
use syn::parse_quote;
use syn::visit::Visit;

#[test]
fn empty_fn_has_cc_1() {
    // Arrange
    let block: syn::Block = parse_quote!({});

    // Act
    let mut visitor = ComplexityVisitor::new();
    visitor.visit_block(&block);

    // Assert
    assert_eq!(visitor.complexity, 1);
}

#[test]
fn single_if_adds_1() {
    // Arrange
    let block: syn::Block = parse_quote!({ if true {} });

    // Act
    let mut visitor = ComplexityVisitor::new();
    visitor.visit_block(&block);

    // Assert
    assert_eq!(visitor.complexity, 2);
}

#[test]
fn if_else_adds_2() {
    // Arrange
    let block: syn::Block = parse_quote!({
        if true {
        } else {
        }
    });

    // Act
    let mut visitor = ComplexityVisitor::new();
    visitor.visit_block(&block);

    // Assert
    assert_eq!(visitor.complexity, 2);
}

#[test]
fn if_else_if_adds_3() {
    // Arrange
    let block: syn::Block = parse_quote!({
        if true {
        } else if false {
        } else {
        }
    });

    // Act
    let mut visitor = ComplexityVisitor::new();
    visitor.visit_block(&block);

    // Assert
    // first if: +1, else if: +1 => total: 3
    assert_eq!(visitor.complexity, 3);
}

#[test]
fn while_loop_adds_1() {
    // Arrange
    let block: syn::Block = parse_quote!({
        while true {
            break;
        }
    });

    // Act
    let mut visitor = ComplexityVisitor::new();
    visitor.visit_block(&block);

    // Assert
    // while: +1, break: +1 => total: 3
    assert_eq!(visitor.complexity, 3);
}

#[test]
fn for_loop_adds_1() {
    // Arrange
    let block: syn::Block = parse_quote!({ for _ in 0..10 {} });

    // Act
    let mut visitor = ComplexityVisitor::new();
    visitor.visit_block(&block);

    // Assert
    assert_eq!(visitor.complexity, 2);
}

#[test]
fn loop_adds_1() {
    // Arrange
    let block: syn::Block = parse_quote!({
        loop {
            break;
        }
    });

    // Act
    let mut visitor = ComplexityVisitor::new();
    visitor.visit_block(&block);

    // Assert
    // loop: +1, break: +1 => total: 3
    assert_eq!(visitor.complexity, 3);
}

#[test]
fn match_with_three_arms_adds_3() {
    // Arrange
    let block: syn::Block = parse_quote!({
        match x {
            1 => {}
            2 => {}
            _ => {}
        }
    });

    // Act
    let mut visitor = ComplexityVisitor::new();
    visitor.visit_block(&block);

    // Assert
    // match: +1, 2 extra arms: +2 => total: 4
    assert_eq!(visitor.complexity, 4);
}

#[test]
fn boolean_and_adds_1() {
    // Arrange
    let block: syn::Block = parse_quote!({ if a && b {} });

    // Act
    let mut visitor = ComplexityVisitor::new();
    visitor.visit_block(&block);

    // Assert
    // if: +1, &&: +1 => total: 3
    assert_eq!(visitor.complexity, 3);
}

#[test]
fn try_operator_adds_1() {
    // Arrange
    let block: syn::Block = parse_quote!({
        let _ = foo()?;
    });

    // Act
    let mut visitor = ComplexityVisitor::new();
    visitor.visit_block(&block);

    // Assert
    assert_eq!(visitor.complexity, 2);
}

#[test]
fn return_adds_1() {
    // Arrange
    let block: syn::Block = parse_quote!({
        return 42;
    });

    // Act
    let mut visitor = ComplexityVisitor::new();
    visitor.visit_block(&block);

    // Assert
    assert_eq!(visitor.complexity, 2);
}

#[test]
fn break_adds_1() {
    // Arrange
    let block: syn::Block = parse_quote!({
        loop {
            break;
        }
    });

    // Act
    let mut visitor = ComplexityVisitor::new();
    visitor.visit_block(&block);

    // Assert
    // loop: +1, break: +1 => total: 3
    assert_eq!(visitor.complexity, 3);
}

#[test]
fn continue_adds_1() {
    // Arrange
    let block: syn::Block = parse_quote!({
        loop {
            continue;
        }
    });

    // Act
    let mut visitor = ComplexityVisitor::new();
    visitor.visit_block(&block);

    // Assert
    // loop: +1, continue: +1 => total: 3
    assert_eq!(visitor.complexity, 3);
}
