// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::fs_walk::FsWalk;
use braintax::traits::walk::Walk;
use tempfile::TempDir;

#[test]
#[ignore]
fn fs_walk_finds_rust_files_in_dir() {
    // This test is ignored by default as it requires filesystem setup.
    // It can be run manually with `cargo test fs_walk -- --ignored`
    // Arrange
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("lib.rs"), "fn foo() {}").unwrap();
    std::fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();
    let walk = FsWalk::new(dir.path());

    // Act
    let files = walk.rust_files().unwrap();

    // Assert
    assert_eq!(files.len(), 2);
}

#[test]
#[ignore]
fn fs_walk_excludes_target_dir() {
    // This test is ignored by default as it requires filesystem setup.
    let dir = TempDir::new().unwrap();
    let target = dir.path().join("target");
    std::fs::create_dir_all(&target).unwrap();
    std::fs::write(target.join("generated.rs"), "fn generated() {}").unwrap();
    std::fs::write(dir.path().join("lib.rs"), "fn foo() {}").unwrap();
    let walk = FsWalk::new(dir.path());

    // Act
    let files = walk.rust_files().unwrap();

    // Assert
    assert_eq!(files.len(), 1);
}
