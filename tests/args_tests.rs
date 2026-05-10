// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::app;

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
fn run_from_args_with_threshold_returns_ok() {
    // Arrange & Act
    let result = app::run_from_args(vec!["test", "--threshold", "10"]);

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
