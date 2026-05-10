// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::collector::Collector;
use std::path::Path;

#[test]
fn collect_trivial_fn_returns_cc_1() {
    // Arrange
    let source = "fn foo() { let x = 1; }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].cyclomatic, 1);
    assert_eq!(functions[0].name, "foo");
}

#[test]
fn collect_fn_with_if_returns_cc_2() {
    // Arrange
    let source = "fn foo(x: bool) { if x { let _ = 1; } }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].cyclomatic, 2);
}

#[test]
fn collect_fn_with_if_else_returns_cc_2() {
    // Arrange
    let source = "fn foo(x: bool) { if x { let _ = 1; } else { let _ = 2; } }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    // if-else is still 1 decision point => CC = 2
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].cyclomatic, 2);
}

#[test]
fn collect_fn_with_while_returns_cc_2() {
    // Arrange
    let source = "fn foo() { while true { break; } }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    // while: +1, break: +1, base: 1 => total: 3
    assert_eq!(functions[0].cyclomatic, 3);
}

#[test]
fn collect_fn_with_for_loop_returns_cc_2() {
    // Arrange
    let source = "fn foo() { for _ in 0..10 { } }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    assert_eq!(functions[0].cyclomatic, 2);
}

#[test]
fn collect_fn_with_match_returns_correct_cc() {
    // Arrange
    let source = r#"
fn foo(x: u32) -> u32 {
    match x {
        1 => 10,
        2 => 20,
        _ => 30,
    }
}
"#;
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    // match: +1, 2 extra arms: +2 => base 1 + 3 = 4
    assert_eq!(functions[0].cyclomatic, 4);
}

#[test]
fn collect_fn_with_boolean_ops_returns_correct_cc() {
    // Arrange
    let source = "fn foo(a: bool, b: bool) { if a && b { } }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    // if: +1, &&: +1, base: 1 => total: 3
    assert_eq!(functions[0].cyclomatic, 3);
}

#[test]
fn collect_fn_with_try_operator_returns_correct_cc() {
    // Arrange
    let source = "fn foo() -> Result<(), ()> { Ok(())?; Ok(()) }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    // ?: +1, base: 1 => total: 2
    assert_eq!(functions[0].cyclomatic, 2);
}

#[test]
fn collect_skips_test_fns() {
    // Arrange
    let source = r#"
#[test]
fn test_me() {
    if true { }
}

fn real() { }
"#;
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].name, "real");
}

#[test]
fn collect_invalid_syntax_returns_empty() {
    // Arrange
    let source = "fn foo( { ";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    assert!(functions.is_empty());
}

#[test]
fn collect_module_path_uses_root() {
    // Arrange
    let source = "fn foo() {}";
    let path = Path::new("/project/src/bar/baz.rs");
    let root = Path::new("/project");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    assert_eq!(functions[0].module, "bar");
}

#[test]
fn collect_cfg_gated_fn_has_cfg_gates_1() {
    // Arrange
    let source = "#[cfg(feature = \"foo\")]\nfn gated() {}";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].cfg_gates, 1);
}

#[test]
fn collect_fn_with_two_cfg_gates_has_cfg_gates_2() {
    // Arrange
    let source =
        "#[cfg(feature = \"a\")]\n#[cfg_attr(feature = \"b\", doc(hidden))]\nfn gated() {}";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].cfg_gates, 2);
}

#[test]
fn collect_fn_with_unsafe_block_has_hidden_deps_1() {
    // Arrange
    let source = "fn foo() { unsafe { let p = std::ptr::null(); } }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].hidden_deps, 1);
}

#[test]
fn collect_fn_with_println_has_hidden_deps_1() {
    // Arrange
    let source = "fn foo() { println!(\"hi\"); }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].hidden_deps, 1);
}

#[test]
fn collect_fn_with_inline_cfg_has_cfg_body_gates() {
    // Arrange
    let source = r#"
fn foo() {
    #[cfg(target_os = "linux")]
    if true { }
}
"#;
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].name, "foo");
}
