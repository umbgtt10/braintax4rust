// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax_cli::invocation::args::Args;
use std::ffi::OsString;

#[test]
fn config_has_default_path() {
    // Arrange
    let args = Args::parse_from_args(vec!["test"]);

    // Act
    let config = args.config();

    // Assert
    assert_eq!(config.path.as_str(), ".");
    assert!(!config.json);
    assert!(config.threshold.is_none());
    assert!(config.max_avg_braintax.is_none());
    assert_eq!(config.top, 10);
}

#[test]
fn config_parses_json_flag() {
    // Arrange
    let args = Args::parse_from_args(vec!["test", "--json"]);

    // Act
    let config = args.config();

    // Assert
    assert!(config.json);
}

#[test]
fn config_parses_max_avg_braintax() {
    // Arrange
    let args = Args::parse_from_args(vec!["test", "--max-avg-braintax", "9.25"]);

    // Act
    let config = args.config();

    // Assert
    assert_eq!(config.max_avg_braintax, Some(9.25));
}

#[test]
fn config_parses_threshold() {
    // Arrange
    let args = Args::parse_from_args(vec!["test", "--threshold", "42"]);

    // Act
    let config = args.config();

    // Assert
    assert_eq!(config.threshold, Some(42));
}

#[test]
fn config_shows_at_least_one_function_for_a_top_of_zero() {
    // Arrange
    let args = Args::parse_from_args(vec!["test", "--top", "0"]);

    // Act
    let config = args.config();

    // Assert
    assert_eq!(config.top, 1);
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
