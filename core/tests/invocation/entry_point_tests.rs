// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::invocation::config::Config;
use braintax::invocation::entry_point::EntryPoint;
use std::ffi::OsString;
use std::fs;
use std::process::ExitCode;
use tempfile::TempDir;

#[test]
fn compose_with_empty_dir_returns_error() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let config = Config::new(dir.path().to_path_buf(), false, None, None, 10);

    // Act
    let result = EntryPoint::compose(config).run();

    // Assert
    assert!(result.is_err());
}

#[test]
fn compose_with_valid_dir_returns_success() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(
        src.join("lib.rs"),
        "pub fn greet() -> &'static str { \"hello\" }\n",
    )
    .unwrap();
    let config = Config::new(dir.path().to_path_buf(), false, None, None, 10);

    // Act
    let exit_code = EntryPoint::compose(config).run().unwrap();

    // Assert
    assert_eq!(exit_code, ExitCode::SUCCESS);
}

#[test]
fn run_against_a_directory_that_does_not_exist_returns_failure() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let missing = dir.path().join("missing");
    let args = vec![
        OsString::from("cargo-braintax4rust"),
        missing.into_os_string(),
    ];

    // Act
    let exit_code = EntryPoint::run(args);

    // Assert
    assert_eq!(exit_code, ExitCode::FAILURE);
}

#[test]
fn run_from_args_with_defaults_returns_ok() {
    // Arrange & Act
    let result = EntryPoint::run_from_args(vec!["test"]);

    // Assert
    assert!(result.is_ok());
}

#[test]
fn run_from_args_with_json_flag_returns_ok() {
    // Arrange & Act
    let result = EntryPoint::run_from_args(vec!["test", "--json"]);

    // Assert
    assert!(result.is_ok());
}

#[test]
fn run_from_args_with_max_complexity_alias_returns_ok() {
    // Arrange & Act
    let result = EntryPoint::run_from_args(vec!["test", "--threshold", "15"]);

    // Assert
    assert!(result.is_ok());
}

#[test]
fn run_from_args_with_threshold_returns_ok() {
    // Arrange & Act
    let result = EntryPoint::run_from_args(vec!["test", "--threshold", "10"]);

    // Assert
    assert!(result.is_ok());
}

// Without the strip, `braintax4rust` would be read as the path to analyse,
// which does not exist, and the run would fail.
#[test]
fn run_with_the_name_cargo_repeats_returns_success() {
    // Arrange
    let args = ["cargo-braintax4rust", "braintax4rust"]
        .map(OsString::from)
        .to_vec();

    // Act
    let exit_code = EntryPoint::run(args);

    // Assert
    assert_eq!(exit_code, ExitCode::SUCCESS);
}
