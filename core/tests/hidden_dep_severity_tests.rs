// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::hidden_dep_severity::HiddenDepSeverity;

#[test]
fn severity_unsafe_returns_8() {
    // Arrange
    let severity = HiddenDepSeverity::new();

    // Act
    let result = severity.severity("unsafe");

    // Assert
    assert_eq!(result, 8.0);
}

#[test]
fn severity_process_exit_returns_6() {
    // Arrange
    let severity = HiddenDepSeverity::new();

    // Act
    let result = severity.severity("process::exit");

    // Assert
    assert_eq!(result, 6.0);
}

#[test]
fn severity_process_abort_returns_6() {
    // Arrange
    let severity = HiddenDepSeverity::new();

    // Act
    let result = severity.severity("process::abort");

    // Assert
    assert_eq!(result, 6.0);
}

#[test]
fn severity_fs_read_returns_5() {
    // Arrange
    let severity = HiddenDepSeverity::new();

    // Act
    let result = severity.severity("fs::read");

    // Assert
    assert_eq!(result, 5.0);
}

#[test]
fn severity_instant_now_returns_4() {
    // Arrange
    let severity = HiddenDepSeverity::new();

    // Act
    let result = severity.severity("Instant::now");

    // Assert
    assert_eq!(result, 4.0);
}

#[test]
fn severity_random_returns_4() {
    // Arrange
    let severity = HiddenDepSeverity::new();

    // Act
    let result = severity.severity("random");

    // Assert
    assert_eq!(result, 4.0);
}

#[test]
fn severity_rand_random_returns_4() {
    // Arrange
    let severity = HiddenDepSeverity::new();

    // Act
    let result = severity.severity("rand::random");

    // Assert
    assert_eq!(result, 4.0);
}

#[test]
fn severity_rand_thread_rng_returns_4() {
    // Arrange
    let severity = HiddenDepSeverity::new();

    // Act
    let result = severity.severity("rand::thread_rng");

    // Assert
    assert_eq!(result, 4.0);
}

#[test]
fn severity_env_var_returns_3() {
    // Arrange
    let severity = HiddenDepSeverity::new();

    // Act
    let result = severity.severity("env::var");

    // Assert
    assert_eq!(result, 3.0);
}

#[test]
fn severity_thread_sleep_returns_3() {
    // Arrange
    let severity = HiddenDepSeverity::new();

    // Act
    let result = severity.severity("thread::sleep");

    // Assert
    assert_eq!(result, 3.0);
}

#[test]
fn severity_println_returns_2() {
    // Arrange
    let severity = HiddenDepSeverity::new();

    // Act
    let result = severity.severity("println");

    // Assert
    assert_eq!(result, 2.0);
}

#[test]
fn severity_unknown_label_returns_default_4() {
    // Arrange
    let severity = HiddenDepSeverity::new();

    // Act
    let result = severity.severity("totally_unknown_label");

    // Assert
    assert_eq!(result, 4.0);
}
