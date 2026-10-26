// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::traits::source_tree::SourceTree;
use braintax_cli::system::portable_path::PortablePath;
use braintax_cli::system::walkdir_source_tree::WalkdirSourceTree;
use std::fs;
use tempfile::TempDir;

// A file name that is not UTF-8 can only be made on Linux: Windows names are
// UTF-16, and macOS's file system refuses a name that is not valid UTF-8. The
// listing carries it lossily, so the file it names cannot be read.
#[cfg(target_os = "linux")]
mod not_utf8 {
    use braintax::traits::source_tree::SourceTree;
    use braintax_cli::system::portable_path::PortablePath;
    use braintax_cli::system::walkdir_source_tree::WalkdirSourceTree;
    use std::ffi::OsStr;
    use std::fs;
    use std::os::unix::ffi::OsStrExt;
    use tempfile::TempDir;

    #[test]
    fn read_source_of_a_listed_file_whose_name_is_not_utf8_fails_naming_it() {
        // Arrange
        let dir = TempDir::new().unwrap();
        fs::write(
            dir.path().join(OsStr::from_bytes(b"bad\xff.rs")),
            "pub fn f() {}",
        )
        .unwrap();
        let tree = WalkdirSourceTree::new();
        let files = tree.rust_files(&PortablePath::of(dir.path())).unwrap();

        // Act
        let failure = tree.read_source(&files[0]).unwrap_err();

        // Assert
        assert!(failure.to_string().ends_with("bad\u{fffd}.rs"), "{failure}");
    }
}

#[test]
fn read_source_of_a_file_on_disk_returns_its_contents() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("lib.rs");
    fs::write(&file, "fn foo() {}").unwrap();

    // Act
    let source = WalkdirSourceTree::new().read_source(&PortablePath::of(&file));

    // Assert
    assert_eq!(source.unwrap(), "fn foo() {}");
}

#[test]
fn read_source_of_a_file_that_does_not_exist_names_it() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let file = PortablePath::of(&dir.path().join("missing.rs"));

    // Act
    let failure = WalkdirSourceTree::new().read_source(&file).unwrap_err();

    // Assert
    assert_eq!(failure.to_string(), format!("failed to read {file}"));
}

#[test]
fn rust_files_lists_every_rust_file_below_the_root_and_nothing_else() {
    // Arrange
    let dir = TempDir::new().unwrap();
    fs::create_dir_all(dir.path().join("src").join("deep")).unwrap();
    fs::write(dir.path().join("src").join("lib.rs"), "").unwrap();
    fs::write(dir.path().join("src").join("deep").join("mod.rs"), "").unwrap();
    fs::write(dir.path().join("src").join("notes.txt"), "").unwrap();
    let root = PortablePath::of(dir.path());

    // Act
    let mut files = WalkdirSourceTree::new().rust_files(&root).unwrap();
    files.sort_by(|a, b| a.as_str().cmp(b.as_str()));

    // Assert
    assert_eq!(
        files,
        vec![
            PortablePath::of(&dir.path().join("src").join("deep").join("mod.rs")),
            PortablePath::of(&dir.path().join("src").join("lib.rs")),
        ]
    );
}

#[test]
fn rust_files_of_a_root_that_does_not_exist_returns_none() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let root = PortablePath::of(&dir.path().join("missing"));

    // Act
    let files = WalkdirSourceTree::new().rust_files(&root).unwrap();

    // Assert
    assert!(files.is_empty());
}

#[test]
fn rust_files_of_a_root_that_is_a_rust_file_returns_it() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("lib.rs");
    fs::write(&file, "").unwrap();

    // Act
    let files = WalkdirSourceTree::new()
        .rust_files(&PortablePath::of(&file))
        .unwrap();

    // Assert
    assert_eq!(files, vec![PortablePath::of(&file)]);
}
