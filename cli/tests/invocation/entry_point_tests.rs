// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::invocation::config::Config;
use braintax_cli::invocation::entry_point::EntryPoint;
use braintax_cli::system::portable_path::PortablePath;
use std::ffi::OsString;
use std::fs;
use std::process::ExitCode;
use tempfile::TempDir;

#[test]
fn compose_with_empty_dir_returns_error() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let config = Config {
        path: PortablePath::of(dir.path()),
        json: false,
        threshold: None,
        max_avg_braintax: None,
        top: 10,
    };

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
    let config = Config {
        path: PortablePath::of(dir.path()),
        json: false,
        threshold: None,
        max_avg_braintax: None,
        top: 10,
    };

    // Act
    let outcome = EntryPoint::compose(config).run().unwrap();

    // Assert
    assert!(outcome.passes);
}

// 2 is an error of the tool's own, as it is clap's for a usage error; 1 stays
// the verdict on a bound not held.
#[test]
fn run_against_a_directory_that_does_not_exist_returns_exit_code_2() {
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
    assert_eq!(exit_code, ExitCode::from(2));
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

#[test]
fn run_with_a_bound_breached_returns_exit_code_1() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(
        src.join("lib.rs"),
        "pub fn greet() -> &'static str { \"hello\" }\n",
    )
    .unwrap();
    let args = vec![
        OsString::from("cargo-braintax4rust"),
        dir.path().as_os_str().to_os_string(),
        OsString::from("--max-avg-braintax"),
        OsString::from("0.5"),
    ];

    // Act
    let exit_code = EntryPoint::run(args);

    // Assert
    assert_eq!(exit_code, ExitCode::from(1));
}

// Not UTF-8, so reading it fails: a failure of the run, not of a bound.
#[test]
fn run_with_a_source_file_it_cannot_read_returns_exit_code_2() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("lib.rs"), [0xff, 0xfe, 0xfd]).unwrap();
    let args = vec![
        OsString::from("cargo-braintax4rust"),
        dir.path().as_os_str().to_os_string(),
    ];

    // Act
    let exit_code = EntryPoint::run(args);

    // Assert
    assert_eq!(exit_code, ExitCode::from(2));
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
