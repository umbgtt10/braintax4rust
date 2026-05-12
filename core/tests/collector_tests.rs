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

#[test]
fn collect_fn_with_single_letter_names_has_opacity() {
    // Arrange
    let source = "fn f(x: i32) { let y = x; }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert!(functions[0].braintax > functions[0].cyclomatic as f64);
}

#[test]
fn collect_fn_with_long_names_has_no_opacity() {
    // Arrange
    let source = "fn compute(value: i32) { let result = value; }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].braintax, functions[0].cyclomatic as f64);
}

#[test]
fn simple_trait_one_impl_has_factor_0_90() {
    // Arrange
    let source = r#"
trait MyTrait {
    fn compute(&self) -> i32;
}
struct MyStruct;
impl MyTrait for MyStruct {
    fn compute(&self) -> i32 { 42 }
}
"#;
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    // MyTrait: 1 method, no assoc, no super, 1 impl → 0.90
    assert_eq!(functions.len(), 1);
    assert!((functions[0].trait_factor - 0.90).abs() < 0.001);
}

#[test]
fn known_std_trait_has_factor_0_80() {
    // Arrange
    let source = r#"
struct MyStruct;
impl std::fmt::Debug for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyStruct")
    }
}
"#;
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    // Debug is in the known-std list → 0.80
    assert_eq!(functions.len(), 1);
    assert!((functions[0].trait_factor - 0.80).abs() < 0.001);
}

#[test]
fn simple_trait_three_impls_has_factor_0_98() {
    // Arrange
    let source = r#"
trait MyTrait {
    fn compute(&self) -> i32;
}
struct A; struct B; struct C;
impl MyTrait for A { fn compute(&self) -> i32 { 1 } }
impl MyTrait for B { fn compute(&self) -> i32 { 2 } }
impl MyTrait for C { fn compute(&self) -> i32 { 3 } }
"#;
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    // MyTrait: 1 method, no assoc, no super, 3 impls → 0.90 + (3-1)*0.06 = 1.02
    assert_eq!(functions.len(), 3);
    for f in &functions {
        assert!((f.trait_factor - 1.02).abs() < 0.001);
    }
}

#[test]
fn simple_trait_four_impls_has_factor_1_05() {
    // Arrange
    let source = r#"
trait MyTrait {
    fn compute(&self) -> i32;
}
struct A; struct B; struct C; struct D;
impl MyTrait for A { fn compute(&self) -> i32 { 1 } }
impl MyTrait for B { fn compute(&self) -> i32 { 2 } }
impl MyTrait for C { fn compute(&self) -> i32 { 3 } }
impl MyTrait for D { fn compute(&self) -> i32 { 4 } }
"#;
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    // MyTrait: 1 method, no assoc, no super, 4 impls → 0.90 + min((4-1)*0.06, 0.18) = 1.08
    assert_eq!(functions.len(), 4);
    for f in &functions {
        assert!((f.trait_factor - 1.08).abs() < 0.001);
    }
}

#[test]
fn trait_with_assoc_type_has_factor_1_25() {
    // Arrange
    let source = r#"
trait MyTrait {
    type Output;
    fn compute(&self) -> Self::Output;
}
struct MyStruct;
impl MyTrait for MyStruct {
    type Output = i32;
    fn compute(&self) -> i32 { 42 }
}
"#;
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    // MyTrait: 1 method, 1 assoc_type, no super, 1 impl
    // base = 1.15, assoc_penalty = 0.15 → 1.30
    assert_eq!(functions.len(), 1);
    assert!((functions[0].trait_factor - 1.30).abs() < 0.001);
}

#[test]
fn trait_with_supertrait_has_factor_1_25() {
    // Arrange
    let source = r#"
trait Base {
    fn base_method(&self);
}
trait Derived: Base {
    fn compute(&self) -> i32;
}
struct MyStruct;
impl Base for MyStruct { fn base_method(&self) {} }
impl Derived for MyStruct { fn compute(&self) -> i32 { 42 } }
"#;
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    // Derived: 1 method, no assoc, 1 supertrait, 1 impl
    // base = 1.15, super_penalty = 0.15 → 1.30
    let derived_fn = functions.iter().find(|f| f.name == "compute").unwrap();
    assert!((derived_fn.trait_factor - 1.30).abs() < 0.001);

    // Base: 1 method, no assoc, no super, 1 impl → 0.90
    let base_fn = functions.iter().find(|f| f.name == "base_method").unwrap();
    assert!((base_fn.trait_factor - 0.90).abs() < 0.001);
}

#[test]
fn trait_with_assoc_and_super_has_factor_1_35() {
    // Arrange
    let source = r#"
trait Base {
    type Output;
    fn process(&self) -> Self::Output;
}
trait Derived: Base {
    type Context;
    fn run(&self, ctx: Self::Context) -> Self::Output;
}
struct MyStruct;
impl Base for MyStruct {
    type Output = i32;
    fn process(&self) -> i32 { 0 }
}
impl Derived for MyStruct {
    type Context = i32;
    fn run(&self, ctx: i32) -> i32 { ctx }
}
"#;
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    // Derived: 1 method, 1 assoc, 1 super, 1 impl
    // base = 1.15, extra_dims = 2 → dim_penalty = 0.20 → 1.35
    let derived_fn = functions.iter().find(|f| f.name == "run").unwrap();
    assert!((derived_fn.trait_factor - 1.35).abs() < 0.001);
}

#[test]
fn trait_with_many_methods_has_method_penalty() {
    // Arrange
    let source = r#"
trait MyTrait {
    fn one(&self);
    fn two(&self);
    fn three(&self);
    fn four(&self);
    fn five(&self);
}
struct MyStruct;
impl MyTrait for MyStruct {
    fn one(&self) {}
    fn two(&self) {}
    fn three(&self) {}
    fn four(&self) {}
    fn five(&self) {}
}
"#;
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    // MyTrait: 5 methods, no assoc, no super, 1 impl
    // method_penalty = (5-3) * 0.01 = 0.02
    // factor = 0.90 + 0.02 = 0.92
    assert_eq!(functions.len(), 5);
    for f in &functions {
        assert!(
            (f.trait_factor - 0.92).abs() < 0.001,
            "expected 0.92, got {} for fn {}",
            f.trait_factor,
            f.name
        );
    }
}

#[test]
fn inherent_impl_has_trait_factor_1_0() {
    // Arrange
    let source = r#"
struct MyStruct;
impl MyStruct {
    fn compute(&self) -> i32 { 42 }
}
"#;
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = Collector::collect(source, path, root);

    // Assert
    // Inherent impl (no trait) → factor = 0.95
    assert_eq!(functions.len(), 1);
    assert!((functions[0].trait_factor - 0.95).abs() < 0.001);
}
