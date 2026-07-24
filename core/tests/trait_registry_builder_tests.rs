// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use braintax::trait_registry_builder::TraitRegistryBuilder;

#[test]
fn build_trait_defined_in_one_file_impl_in_another_records_shape() {
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
    let files = vec![
        (
            PathBuf::from("src/traits/my_trait.rs"),
            trait_source.to_string(),
        ),
        (PathBuf::from("src/my_struct.rs"), impl_source.to_string()),
    ];

    // Act
    let traits = TraitRegistryBuilder::new().build(&files);

    // Assert
    let info = traits.get("MyTrait").expect("MyTrait should be registered");
    assert_eq!(info.assoc_types, 1);
    assert_eq!(info.impl_count, 1);
}

#[test]
fn build_impls_across_multiple_files_sum_impl_count() {
    // Arrange
    let trait_source = "trait Greet { fn hello(&self); }";
    let impl_a = r#"
struct A;
impl Greet for A { fn hello(&self) {} }
"#;
    let impl_b = r#"
struct B;
impl Greet for B { fn hello(&self) {} }
"#;
    let files = vec![
        (
            PathBuf::from("src/traits/greet.rs"),
            trait_source.to_string(),
        ),
        (PathBuf::from("src/a.rs"), impl_a.to_string()),
        (PathBuf::from("src/b.rs"), impl_b.to_string()),
    ];

    // Act
    let traits = TraitRegistryBuilder::new().build(&files);

    // Assert
    assert_eq!(traits.get("Greet").unwrap().impl_count, 2);
}

#[test]
fn build_trait_with_no_impls_registers_with_zero_impl_count() {
    // Arrange
    let trait_source = "trait Unused { fn never_called(&self); }";
    let files = vec![(
        PathBuf::from("src/traits/unused.rs"),
        trait_source.to_string(),
    )];

    // Act
    let traits = TraitRegistryBuilder::new().build(&files);

    // Assert
    assert_eq!(traits.get("Unused").unwrap().impl_count, 0);
}
