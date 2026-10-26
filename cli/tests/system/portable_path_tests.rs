// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax_cli::system::portable_path::PortablePath;
use std::path::{MAIN_SEPARATOR, Path};

#[test]
fn of_a_backslash_keeps_it_only_where_it_is_not_a_separator() {
    // Arrange
    let path = Path::new("core\\lib.rs");

    // Act
    let portable = PortablePath::of(path);

    // Assert
    let expected = if MAIN_SEPARATOR == '\\' {
        "core/lib.rs"
    } else {
        "core\\lib.rs"
    };
    assert_eq!(portable.as_str(), expected);
}

#[test]
fn of_a_path_joined_by_the_platform_uses_forward_separators() {
    // Arrange
    let path = Path::new("core").join("src").join("lib.rs");

    // Act
    let portable = PortablePath::of(&path);

    // Assert
    assert_eq!(portable.as_str(), "core/src/lib.rs");
}

#[test]
fn of_a_path_written_with_forward_separators_keeps_it() {
    // Arrange
    let path = Path::new("./core/src/lib.rs");

    // Act
    let portable = PortablePath::of(path);

    // Assert
    assert_eq!(portable.as_str(), "./core/src/lib.rs");
}
