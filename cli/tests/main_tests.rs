// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::process::Command;
use tempfile::TempDir;

// The exit code is the binary's interface: 0 a pass, 1 a bound not held, 2 a
// run that gave no verdict -- an error of the tool's own or, from clap, of its
// usage.
#[test]
fn binary_exits_1_on_a_breached_bound() {
    // Arrange
    let mut command = Command::new("cargo");
    command.args(["run", "--", "src", "--max-avg-braintax", "0"]);

    // Act
    let output = command.output().expect("failed to run cargo");

    // Assert
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn binary_exits_2_on_a_usage_error() {
    // Arrange
    let mut command = Command::new("cargo");
    command.args(["run", "--", "--no-such-flag"]);

    // Act
    let output = command.output().expect("failed to run cargo");

    // Assert
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn binary_exits_2_on_an_error_of_its_own() {
    // Arrange
    let dir = TempDir::new().unwrap();
    let mut command = Command::new("cargo");
    command.args(["run", "--"]).arg(dir.path().join("missing"));

    // Act
    let output = command.output().expect("failed to run cargo");

    // Assert
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("error: no Rust source files found"));
}

#[test]
fn binary_prints_help() {
    // Arrange & Act
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("failed to run cargo");

    // Assert
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Estimate the cognitive tax of Rust code"));
    assert!(stdout.contains("--json"));
    assert!(stdout.contains("--threshold"));
    assert!(stdout.contains("--top"));
}

#[test]
fn binary_prints_version() {
    // Arrange
    let mut command = Command::new("cargo");
    command.args(["run", "--", "--version"]);

    // Act
    let output = command.output().expect("failed to run cargo");

    // Assert
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0.16.0"));
}
