// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::config::Config;

#[test]
fn config_has_default_path() {
    // Arrange & Act
    let args = braintax::args::Args::parse_from_args(vec!["test"]);

    // Act
    let config = Config::from_args(args);

    // Assert
    assert_eq!(config.path.to_string_lossy(), ".");
    assert!(!config.json);
    assert!(config.threshold.is_none());
    assert!(config.max_avg_braintax.is_none());
    assert_eq!(config.top, 10);
}

#[test]
fn config_parses_json_flag() {
    // Arrange & Act
    let args = braintax::args::Args::parse_from_args(vec!["test", "--json"]);

    // Act
    let config = Config::from_args(args);

    // Assert
    assert!(config.json);
}

#[test]
fn config_parses_threshold() {
    // Arrange & Act
    let args = braintax::args::Args::parse_from_args(vec!["test", "--threshold", "42"]);

    // Act
    let config = Config::from_args(args);

    // Assert
    assert_eq!(config.threshold, Some(42));
}

#[test]
fn config_parses_max_avg_braintax() {
    // Arrange & Act
    let args = braintax::args::Args::parse_from_args(vec!["test", "--max-avg-braintax", "9.25"]);

    // Act
    let config = Config::from_args(args);

    // Assert
    assert_eq!(config.max_avg_braintax, Some(9.25));
}
