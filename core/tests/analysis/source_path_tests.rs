// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::analysis::source_path::SourcePath;

fn path(text: &str) -> SourcePath {
    SourcePath::new(String::from(text))
}

#[test]
fn as_str_returns_the_text_as_given() {
    // Arrange
    let source = path("./core//src/lib.rs");

    // Act
    let text = source.as_str();

    // Assert
    assert_eq!(text, "./core//src/lib.rs");
}

#[test]
fn file_name_of_a_directory_with_a_trailing_separator_returns_its_name() {
    // Arrange & Act
    let name = path("core/").file_name().map(String::from);

    // Assert
    assert_eq!(name.as_deref(), Some("core"));
}

#[test]
fn file_name_of_a_file_returns_its_last_component() {
    // Arrange & Act
    let name = path("/work/src/lib.rs").file_name().map(String::from);

    // Assert
    assert_eq!(name.as_deref(), Some("lib.rs"));
}

#[test]
fn file_name_of_a_parent_directory_returns_none() {
    // Arrange & Act
    let name = path("core/..").file_name().map(String::from);

    // Assert
    assert_eq!(name, None);
}

#[test]
fn file_name_of_the_current_directory_returns_none() {
    // Arrange & Act
    let name = path(".").file_name().map(String::from);

    // Assert
    assert_eq!(name, None);
}

#[test]
fn file_name_of_the_root_returns_none() {
    // Arrange & Act
    let name = path("/").file_name().map(String::from);

    // Assert
    assert_eq!(name, None);
}

#[test]
fn file_name_skips_a_trailing_current_directory() {
    // Arrange & Act
    let name = path("./core/.").file_name().map(String::from);

    // Assert
    assert_eq!(name.as_deref(), Some("core"));
}

#[test]
fn fmt_writes_the_text_as_given() {
    // Arrange
    let source = path("./core/src");

    // Act
    let text = source.to_string();

    // Assert
    assert_eq!(text, "./core/src");
}

#[test]
fn strip_prefix_compares_by_component_not_by_text() {
    // Arrange
    let source = path("core/./src//lib.rs/");

    // Act
    let rest = source.strip_prefix(&path("core/.")).map(String::from);

    // Assert
    assert_eq!(rest.as_deref(), Some("src//lib.rs"));
}

#[test]
fn strip_prefix_matches_whole_components_only() {
    // Arrange
    let source = path("/core-cli/src/lib.rs");

    // Act
    let rest = source.strip_prefix(&path("/core")).map(String::from);

    // Assert
    assert_eq!(rest, None);
}

#[test]
fn strip_prefix_of_a_path_outside_the_base_returns_none() {
    // Arrange
    let source = path("src/lib.rs");

    // Act
    let rest = source.strip_prefix(&path(".")).map(String::from);

    // Assert
    assert_eq!(rest, None);
}

#[test]
fn strip_prefix_of_a_path_under_the_current_directory_returns_the_rest() {
    // Arrange
    let source = path("./src/bar/baz.rs");

    // Act
    let rest = source.strip_prefix(&path(".")).map(String::from);

    // Assert
    assert_eq!(rest.as_deref(), Some("src/bar/baz.rs"));
}

#[test]
fn strip_prefix_of_a_relative_path_under_the_root_directory_returns_none() {
    // Arrange
    let source = path("project/src/lib.rs");

    // Act
    let rest = source.strip_prefix(&path("/project")).map(String::from);

    // Assert
    assert_eq!(rest, None);
}

#[test]
fn strip_prefix_of_an_absolute_path_returns_the_rest() {
    // Arrange
    let source = path("/project/src/bar/baz.rs");

    // Act
    let rest = source.strip_prefix(&path("/project")).map(String::from);

    // Assert
    assert_eq!(rest.as_deref(), Some("src/bar/baz.rs"));
}

#[test]
fn strip_prefix_of_the_path_itself_returns_nothing_left() {
    // Arrange
    let source = path("core/src");

    // Act
    let rest = source.strip_prefix(&path("core/src/")).map(String::from);

    // Assert
    assert_eq!(rest.as_deref(), Some(""));
}

#[test]
fn strip_prefix_with_a_base_longer_than_the_path_returns_none() {
    // Arrange
    let source = path("core");

    // Act
    let rest = source.strip_prefix(&path("core/src")).map(String::from);

    // Assert
    assert_eq!(rest, None);
}
