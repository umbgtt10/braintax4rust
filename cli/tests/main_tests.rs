// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde_json::{Value, from_str, json};
use std::fs;
use std::process::Command;
use tempfile::TempDir;

// A crate of traits alone, under a directory named for it: Rust files, but no
// function with a body.
fn crate_of_traits_alone() -> TempDir {
    let dir = TempDir::new().unwrap();
    let root = dir.path().join("traits_only");
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"traits_only\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .unwrap();
    fs::write(
        root.join("src").join("lib.rs"),
        "pub trait T {\n    fn f(&self);\n}\n",
    )
    .unwrap();
    dir
}

// The exit code is the binary's interface: 0 a pass, 1 a bound not held, 2 a
// run that gave no verdict -- an error of the tool's own or, from clap, of its
// usage. Rust files with no function with a body are measured, not refused:
// nothing to score, so every figure is 0 and no bound is breached.
#[test]
fn binary_exits_0_on_a_crate_of_traits_alone_and_reports_every_figure_at_0() {
    // Arrange
    let dir = crate_of_traits_alone();
    let mut command = Command::new("cargo");
    command
        .args(["run", "--"])
        .arg(dir.path().join("traits_only"));

    // Act
    let output = command.output().expect("failed to run cargo");

    // Assert
    assert_eq!(output.status.code(), Some(0));
    let expected = [
        format!(
            "cargo-braintax4rust {} -- traits_only",
            env!("CARGO_PKG_VERSION")
        ),
        String::from("══════════════════════════════════════════════"),
        String::new(),
        String::from("  Overall braintax:            0.0"),
        String::from("  Maximum braintax:           0.0"),
        String::from("  Total braintax:             0.0"),
        String::from("  Normalized braintax:        0 / 100"),
        String::new(),
        String::from("Cyclomatic complexity:"),
        String::from("  Total functions:             0"),
        String::from("  Average complexity:         0.0"),
        String::from("  Maximum complexity:         0"),
        String::from("  Total complexity:           0"),
        String::new(),
    ]
    .join("\n");
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected);
}

#[test]
fn binary_exits_0_on_a_crate_of_traits_alone_and_writes_json_with_every_figure_at_0() {
    // Arrange
    let dir = crate_of_traits_alone();
    let mut command = Command::new("cargo");
    command
        .args(["run", "--", "--json"])
        .arg(dir.path().join("traits_only"));

    // Act
    let output = command.output().expect("failed to run cargo");

    // Assert
    assert_eq!(output.status.code(), Some(0));
    let report: Value = from_str(&String::from_utf8_lossy(&output.stdout)).unwrap();
    assert_eq!(
        report,
        json!({
            "version": env!("CARGO_PKG_VERSION"),
            "target": "traits_only",
            "overall": {
                "total_functions": 0,
                "avg_cyclomatic": 0.0,
                "max_cyclomatic": 0,
                "total_cyclomatic": 0,
                "avg_braintax": 0.0,
                "max_braintax": 0.0,
                "total_braintax": 0.0,
                "braintax_normalized": 0
            },
            "modules": [],
            "functions": []
        })
    );
}

#[test]
fn binary_exits_0_on_a_crate_of_traits_alone_under_both_bounds_at_0() {
    // Arrange
    let dir = crate_of_traits_alone();
    let mut command = Command::new("cargo");
    command
        .args(["run", "--", "--threshold", "0", "--max-avg-braintax", "0"])
        .arg(dir.path().join("traits_only"));

    // Act
    let output = command.output().expect("failed to run cargo");

    // Assert
    assert_eq!(output.status.code(), Some(0));
}

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
    assert!(stdout.contains("0.16.1"));
}
