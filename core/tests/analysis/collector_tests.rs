// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::analysis::collector::Collector;
use braintax::counting::function_complexity::FunctionComplexity;
use std::path::{Path, PathBuf};

fn collect(source: &str, path: &Path, root: &Path) -> Vec<FunctionComplexity> {
    let traits = Collector::build_trait_registry(&[(path.to_path_buf(), source.to_string())]);
    Collector::collect(source, path, root, &traits)
}

#[test]
fn build_trait_registry_trait_defined_separately_from_impl_has_real_factor() {
    // Arrange
    let trait_source = r#"
trait MyTrait {
    type Output;
    fn compute(&self) -> Self::Output;
}
"#;
    let impl_source = r#"
struct MyStruct;
impl MyTrait for MyStruct {
    type Output = i32;
    fn compute(&self) -> i32 { 42 }
}
"#;
    let trait_path = PathBuf::from("src/traits/my_trait.rs");
    let impl_path = PathBuf::from("src/my_struct.rs");
    let root = Path::new(".");
    let files = vec![
        (trait_path, trait_source.to_string()),
        (impl_path.clone(), impl_source.to_string()),
    ];

    // Act
    let traits = Collector::build_trait_registry(&files);
    let functions = Collector::collect(impl_source, &impl_path, root, &traits);

    // Assert
    assert_eq!(functions.len(), 1);
    assert!(
        (functions[0].trait_factor - 1.30).abs() < 0.001,
        "expected 1.30 (assoc-type trait, cross-file), got {}",
        functions[0].trait_factor
    );
}

#[test]
fn collect_cfg_gated_fn_has_cfg_gates_1() {
    // Arrange
    let source = "#[cfg(feature = \"foo\")]\nfn gated() {}";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].cfg_gates, 1);
}

#[test]
fn collect_fn_with_boolean_ops_returns_correct_cc() {
    // Arrange
    let source = "fn foo(a: bool, b: bool) { if a && b { } }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions[0].cyclomatic, 3);
}

#[test]
fn collect_fn_with_for_loop_returns_cc_2() {
    // Arrange
    let source = "fn foo() { for _ in 0..10 { } }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions[0].cyclomatic, 2);
}

#[test]
fn collect_fn_with_if_else_returns_cc_2() {
    // Arrange
    let source = "fn foo(x: bool) { if x { let _ = 1; } else { let _ = 2; } }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].cyclomatic, 2);
}

#[test]
fn collect_fn_with_if_returns_cc_2() {
    // Arrange
    let source = "fn foo(x: bool) { if x { let _ = 1; } }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].cyclomatic, 2);
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
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].name, "foo");
}

#[test]
fn collect_fn_with_long_names_has_no_opacity() {
    // Arrange
    let source = "fn compute(value: i32) { let result = value; }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].braintax, functions[0].cyclomatic as f64);
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
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions[0].cyclomatic, 4);
}

#[test]
fn collect_fn_with_println_has_hidden_deps_1() {
    // Arrange
    let source = "fn foo() { println!(\"hi\"); }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].hidden_deps, 1);
    assert_eq!(functions[0].hidden_dep_weight, 2.0);
    assert_eq!(functions[0].hidden_dep_labels, vec!["println".to_string()]);
}

#[test]
fn collect_fn_with_single_letter_names_has_opacity() {
    // Arrange
    let source = "fn f(x: i32) { let y = x; }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert!(functions[0].braintax > functions[0].cyclomatic as f64);
}

#[test]
fn collect_fn_with_try_operator_returns_correct_cc() {
    // Arrange
    let source = "fn foo() -> Result<(), ()> { Ok(())?; Ok(()) }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions[0].cyclomatic, 2);
}

#[test]
fn collect_fn_with_two_cfg_gates_has_cfg_gates_2() {
    // Arrange
    let source =
        "#[cfg(feature = \"a\")]\n#[cfg_attr(feature = \"b\", doc(hidden))]\nfn gated() {}";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = collect(source, path, root);

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
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].hidden_deps, 1);
    assert_eq!(functions[0].hidden_dep_weight, 8.0);
    assert_eq!(functions[0].hidden_dep_labels, vec!["unsafe".to_string()]);
}

#[test]
fn collect_fn_with_while_returns_cc_2() {
    // Arrange
    let source = "fn foo() { while true { break; } }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions[0].cyclomatic, 3);
}

#[test]
fn collect_invalid_syntax_returns_empty() {
    // Arrange
    let source = "fn foo( { ";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = collect(source, path, root);

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
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions[0].module, "bar");
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
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].name, "real");
}

#[test]
fn collect_trivial_fn_returns_cc_1() {
    // Arrange
    let source = "fn foo() { let x = 1; }";
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].cyclomatic, 1);
    assert_eq!(functions[0].name, "foo");
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
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert!((functions[0].trait_factor - 0.95).abs() < 0.001);
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
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert!((functions[0].trait_factor - 0.80).abs() < 0.001);
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
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 4);
    for f in &functions {
        assert!((f.trait_factor - 1.08).abs() < 0.001);
    }
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
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert!((functions[0].trait_factor - 0.90).abs() < 0.001);
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
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 3);
    for f in &functions {
        assert!((f.trait_factor - 1.02).abs() < 0.001);
    }
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
    let functions = collect(source, path, root);

    // Assert
    let derived_fn = functions.iter().find(|f| f.name == "run").unwrap();
    assert!((derived_fn.trait_factor - 1.35).abs() < 0.001);
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
    let functions = collect(source, path, root);

    // Assert
    assert_eq!(functions.len(), 1);
    assert!((functions[0].trait_factor - 1.30).abs() < 0.001);
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
    let functions = collect(source, path, root);

    // Assert
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
fn trait_with_real_supertrait_and_send_has_factor_1_30() {
    // Arrange: a real supertrait (Base) plus a marker (Send) must score the
    // same as the real supertrait alone - Send must not add a second
    // dimension on top of it.
    let source = r#"
trait Base {
    fn base_method(&self);
}
trait Derived: Base + Send {
    fn compute(&self) -> i32;
}
struct MyStruct;
impl Base for MyStruct { fn base_method(&self) {} }
impl Derived for MyStruct { fn compute(&self) -> i32 { 42 } }
"#;
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = collect(source, path, root);

    // Assert
    let derived_fn = functions.iter().find(|f| f.name == "compute").unwrap();
    assert!((derived_fn.trait_factor - 1.30).abs() < 0.001);
}

#[test]
fn trait_with_send_supertrait_has_factor_0_90() {
    // Arrange: Send is a zero-cost auto-trait with no methods and no
    // conceptual surface - bounding a trait by it must not trigger the
    // same dimension penalty a real supertrait would.
    let source = r#"
trait Observer: Send {
    fn notify(&self, event: i32);
}
struct MyObserver;
impl Observer for MyObserver { fn notify(&self, event: i32) {} }
"#;
    let path = Path::new("src/lib.rs");
    let root = Path::new(".");

    // Act
    let functions = collect(source, path, root);

    // Assert
    let notify_fn = functions.iter().find(|f| f.name == "notify").unwrap();
    assert!(
        (notify_fn.trait_factor - 0.90).abs() < 0.001,
        "expected 0.90 (no real dimension), got {}",
        notify_fn.trait_factor
    );
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
    let functions = collect(source, path, root);

    // Assert
    let derived_fn = functions.iter().find(|f| f.name == "compute").unwrap();
    assert!((derived_fn.trait_factor - 1.30).abs() < 0.001);

    let base_fn = functions.iter().find(|f| f.name == "base_method").unwrap();
    assert!((base_fn.trait_factor - 0.90).abs() < 0.001);
}
