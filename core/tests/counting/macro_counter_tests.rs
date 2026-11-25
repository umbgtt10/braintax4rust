// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::counting::macro_counter::MacroCounter;
use syn::Block;
use syn::parse_str;
use syn::visit::Visit;

fn score_block(code: &str) -> u32 {
    let block: Block = parse_str(code).unwrap();
    let mut counter = MacroCounter::new();
    counter.visit_block(&block);
    counter.count()
}

#[test]
fn empty_block_scores_zero() {
    // Arrange & Act
    let score = score_block("{}");

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
fn known_format_macro_scores_zero() {
    // Arrange & Act
    let score = score_block("{ let _ = format!(\"hello\"); }");

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
fn mixed_known_and_unknown_counts_only_unknown() {
    // Arrange & Act
    let score = score_block("{ let _ = vec![1]; my_macro!(); }");

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
fn visit_block_attribute_named_after_a_function_like_macro_scores_3() {
    // Arrange & Act
    let score = score_block("{ #[println] fn inner() {} }");

    // Assert
    assert_eq!(score, 3);
}

#[test]
fn visit_block_built_in_attributes_score_zero() {
    // Arrange
    let built_in = [
        "allow",
        "automatically_derived",
        "cfg",
        "cfg_attr",
        "cold",
        "collapse_debuginfo",
        "crate_name",
        "crate_type",
        "debugger_visualizer",
        "deny",
        "deprecated",
        "doc",
        "expect",
        "export_name",
        "feature",
        "forbid",
        "global_allocator",
        "ignore",
        "inline",
        "instruction_set",
        "link",
        "link_name",
        "link_ordinal",
        "link_section",
        "macro_export",
        "macro_use",
        "must_use",
        "naked",
        "no_builtins",
        "no_implicit_prelude",
        "no_link",
        "no_main",
        "no_mangle",
        "no_std",
        "non_exhaustive",
        "panic_handler",
        "path",
        "proc_macro",
        "proc_macro_attribute",
        "proc_macro_derive",
        "recursion_limit",
        "repr",
        "should_panic",
        "target_feature",
        "test",
        "track_caller",
        "type_length_limit",
        "used",
        "warn",
        "windows_subsystem",
    ];

    // Act & Assert
    for name in built_in {
        let block = format!("{{ #[{name}] fn inner() {{}} }}");
        assert_eq!(score_block(&block), 0, "#[{name}]");
    }
}

#[test]
fn visit_block_derive_of_non_std_traits_scores_3_each() {
    // Arrange & Act
    let score = score_block("{ #[derive(Clone, Serialize, Deserialize)] struct Point; }");

    // Assert
    assert_eq!(score, 6);
}

#[test]
fn visit_block_doc_comments_score_zero() {
    // Arrange & Act
    let score = score_block("{\n    /// One.\n    /// Two.\n    /// Three.\n    fn inner() {}\n}");

    // Assert
    assert_eq!(score, 0);
}

#[test]
fn visit_block_known_lint_and_cfg_attributes_score_zero() {
    // Arrange & Act
    let score = score_block(
        "{ #[allow(unused)] #[deny(warnings)] #[warn(missing_docs)] #[cfg(test)] \
         #[cfg_attr(test, inline)] let x = 1; }",
    );

    // Assert
    assert_eq!(score, 0);
}

#[test]
fn visit_block_unknown_attribute_scores_3() {
    // Arrange & Act
    let score = score_block("{ #[my_attr] fn inner() {} }");

    // Assert
    assert_eq!(score, 3);
}
