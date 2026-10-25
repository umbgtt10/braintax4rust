// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::invocation::args::Args;
use std::ffi::OsString;

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
