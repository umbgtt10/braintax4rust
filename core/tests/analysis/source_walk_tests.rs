// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_source_tree::InMemorySourceTree;
use braintax::analysis::source_path::SourcePath;
use braintax::analysis::source_walk::SourceWalk;
use braintax::traits::walk::Walk;

fn walk(tree: InMemorySourceTree, root: &str) -> SourceWalk {
    SourceWalk::new(Box::new(tree), SourcePath::new(String::from(root)))
}

#[test]
fn rust_files_excludes_a_build_script() {
    // Arrange
    let tree = InMemorySourceTree::new()
        .with_unreadable_file("/crate/build.rs")
        .with_file("/crate/src/lib.rs", "fn foo() {}");
    let walk = walk(tree, "/crate");

    // Act
    let files = walk.rust_files().unwrap();

    // Assert
    assert_eq!(
        files,
        vec![(
            SourcePath::new(String::from("/crate/src/lib.rs")),
            String::from("fn foo() {}")
        )]
    );
}

#[test]
fn rust_files_excludes_every_build_vcs_example_and_bench_directory() {
    // Arrange
    let tree = InMemorySourceTree::new()
        .with_unreadable_file("/crate/.git/hook.rs")
        .with_unreadable_file("/crate/benches/speed.rs")
        .with_unreadable_file("/crate/examples/demo.rs")
        .with_file("/crate/src/lib.rs", "fn foo() {}");
    let walk = walk(tree, "/crate");

    // Act
    let files = walk.rust_files().unwrap();

    // Assert
    assert_eq!(files.len(), 1);
}

#[test]
fn rust_files_excludes_target_dir() {
    // Arrange
    let tree = InMemorySourceTree::new()
        .with_file("/work/src/target/generated.rs", "fn generated() {}")
        .with_file("/work/src/lib.rs", "fn foo() {}");
    let walk = walk(tree, "/work/src");

    // Act
    let files = walk.rust_files().unwrap();

    // Assert
    assert_eq!(files.len(), 1);
}

#[test]
fn rust_files_excludes_tests_dir() {
    // Arrange
    let tree = InMemorySourceTree::new()
        .with_file("/work/src/tests/test_me.rs", "fn test() {}")
        .with_file("/work/src/lib.rs", "fn foo() {}");
    let walk = walk(tree, "/work/src");

    // Act
    let files = walk.rust_files().unwrap();

    // Assert
    assert_eq!(files.len(), 1);
}

#[test]
fn rust_files_finds_rust_files_in_dir() {
    // Arrange
    let tree = InMemorySourceTree::new()
        .with_file("/work/src/lib.rs", "fn foo() {}")
        .with_file("/work/src/main.rs", "fn main() {}");
    let walk = walk(tree, "/work/src");

    // Act
    let files = walk.rust_files().unwrap();

    // Assert
    assert_eq!(files.len(), 2);
}

#[test]
fn rust_files_judges_directories_below_the_root_only() {
    // Arrange
    let tree = InMemorySourceTree::new().with_file("/tests/crate/src/lib.rs", "fn foo() {}");
    let walk = walk(tree, "/tests/crate");

    // Act
    let files = walk.rust_files().unwrap();

    // Assert
    assert_eq!(files.len(), 1);
}

#[test]
fn rust_files_with_a_file_that_cannot_be_read_names_it() {
    // Arrange
    let tree = InMemorySourceTree::new().with_unreadable_file("/crate/src/lib.rs");
    let walk = walk(tree, "/crate");

    // Act
    let failure = walk.rust_files().unwrap_err();

    // Assert
    assert_eq!(failure.to_string(), "failed to read /crate/src/lib.rs");
}
