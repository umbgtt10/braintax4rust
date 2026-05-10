// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::fs_walk::FsWalk;
use braintax::traits::walk::Walk;
use std::fs;
use tempfile::TempDir;

#[test]
fn fs_walk_finds_rust_files_in_dir() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let root = dir.path().join("src");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("lib.rs"), "fn foo() {}").unwrap();
    fs::write(root.join("main.rs"), "fn main() {}").unwrap();
    let walk = FsWalk::new(&root);

    // Act
    let files = walk.rust_files().unwrap();

    // Assert
    assert_eq!(files.len(), 2);
}

#[test]
fn fs_walk_excludes_target_dir() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let root = dir.path().join("src");
    fs::create_dir_all(root.join("target")).unwrap();
    fs::write(
        root.join("target").join("generated.rs"),
        "fn generated() {}",
    )
    .unwrap();
    fs::write(root.join("lib.rs"), "fn foo() {}").unwrap();
    let walk = FsWalk::new(&root);

    // Act
    let files = walk.rust_files().unwrap();

    // Assert
    assert_eq!(files.len(), 1);
}

#[test]
fn fs_walk_excludes_tests_dir() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let root = dir.path().join("src");
    fs::create_dir_all(root.join("tests")).unwrap();
    fs::write(root.join("tests").join("test_me.rs"), "fn test() {}").unwrap();
    fs::write(root.join("lib.rs"), "fn foo() {}").unwrap();
    let walk = FsWalk::new(&root);

    // Act
    let files = walk.rust_files().unwrap();

    // Assert
    assert_eq!(files.len(), 1);
}
