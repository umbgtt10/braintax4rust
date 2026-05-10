// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::process::Command;

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
    // Act
    let output = Command::new("cargo")
        .args(["run", "--", "--version"])
        .output()
        .expect("failed to run cargo");

    // Assert
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0.5.0"));
}
