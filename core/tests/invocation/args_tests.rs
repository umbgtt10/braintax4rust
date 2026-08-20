// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::invocation::app;
use braintax::invocation::args::Args;
use std::ffi::OsString;

#[test]
fn run_from_args_with_defaults_returns_ok() {
    // Arrange & Act
    let result = app::run_from_args(vec!["test"]);

    // Assert
    assert!(result.is_ok());
}

#[test]
fn run_from_args_with_json_flag_returns_ok() {
    // Arrange & Act
    let result = app::run_from_args(vec!["test", "--json"]);

    // Assert
    assert!(result.is_ok());
}

#[test]
fn run_from_args_with_max_complexity_alias_returns_ok() {
    // Arrange & Act
    let result = app::run_from_args(vec!["test", "--threshold", "15"]);

    // Assert
    assert!(result.is_ok());
}

#[test]
fn run_from_args_with_threshold_returns_ok() {
    // Arrange & Act
    let result = app::run_from_args(vec!["test", "--threshold", "10"]);

    // Assert
    assert!(result.is_ok());
}

#[test]
fn without_cargo_subcommand_drops_the_name_cargo_repeats() {
    // Arrange
    let raw = ["cargo-braintax4rust", "braintax4rust", "--json"]
        .map(OsString::from)
        .to_vec();

    // Act
    let forwarded = Args::without_cargo_subcommand(raw);

    // Assert
    assert_eq!(
        forwarded,
        ["cargo-braintax4rust", "--json"].map(OsString::from)
    );
}

#[test]
fn without_cargo_subcommand_keeps_a_path_named_after_the_tool() {
    // Arrange
    let raw = ["cargo-braintax4rust", "braintax4rust", "braintax4rust"]
        .map(OsString::from)
        .to_vec();

    // Act
    let forwarded = Args::without_cargo_subcommand(raw);

    // Assert
    assert_eq!(
        forwarded,
        ["cargo-braintax4rust", "braintax4rust"].map(OsString::from)
    );
}

#[test]
fn without_cargo_subcommand_leaves_a_direct_invocation_untouched() {
    // Arrange
    let raw = ["cargo-braintax4rust", "--json"]
        .map(OsString::from)
        .to_vec();

    // Act
    let forwarded = Args::without_cargo_subcommand(raw);

    // Assert
    assert_eq!(
        forwarded,
        ["cargo-braintax4rust", "--json"].map(OsString::from)
    );
}

#[test]
fn without_cargo_subcommand_on_a_bare_binary_name_returns_it_unchanged() {
    // Arrange
    let raw = vec![OsString::from("cargo-braintax4rust")];

    // Act
    let forwarded = Args::without_cargo_subcommand(raw);

    // Assert
    assert_eq!(forwarded, [OsString::from("cargo-braintax4rust")]);
}
