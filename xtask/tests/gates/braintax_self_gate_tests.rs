// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::process::command_runner_tests::FakeCommandRunner;
use xtask::gates::braintax_self_gate::BraintaxSelfGate;
use xtask::gates::gate::Gate;

fn gate(runner: &FakeCommandRunner) -> BraintaxSelfGate<'_> {
    BraintaxSelfGate::new(
        runner,
        String::from("cargo-braintax4rust"),
        String::from("9.03"),
    )
}

#[test]
fn label_names_the_self_analysis_gate() {
    // Arrange
    let runner = FakeCommandRunner::new();

    // Act
    let label = gate(&runner).label();

    // Assert
    assert_eq!(label, "braintax self-analysis");
}

#[test]
fn run_builds_the_tool_from_this_checkout_rather_than_an_install() {
    // Arrange
    let runner = FakeCommandRunner::new();

    // Act
    let _ = gate(&runner).run();

    // Assert
    let call = &runner.calls()[0];
    assert_eq!(call[0], "run");
    assert!(call.contains(&String::from("cargo-braintax4rust")));
}

// The ceiling is the tool's to enforce, so it has to reach the tool. Judging
// it here instead would put a second implementation of the comparison in the
// gate, free to disagree with the one that ships.
#[test]
fn run_passes_the_ceiling_through_to_the_tool() {
    // Arrange
    let runner = FakeCommandRunner::new();

    // Act
    let _ = gate(&runner).run();

    // Assert
    let call = &runner.calls()[0];
    assert!(call.contains(&String::from("--max-avg-braintax")));
    assert!(call.contains(&String::from("9.03")));
}

#[test]
fn run_with_a_non_zero_exit_code_names_the_ceiling_it_breached() {
    // Arrange
    let runner = FakeCommandRunner::new().with_streaming_code(Some(1));

    // Act
    let result = gate(&runner).run();

    // Assert
    assert_eq!(
        result,
        Err(String::from("avg braintax exceeds the ceiling of 9.03"))
    );
}

#[test]
fn run_with_a_zero_exit_code_returns_ok() {
    // Arrange
    let runner = FakeCommandRunner::new();

    // Act
    let result = gate(&runner).run();

    // Assert
    assert!(result.is_ok());
}

// None means the process was signalled rather than exiting. That is a failure,
// not a pass, and the arm that treats it as one would be easy to write.
#[test]
fn run_with_no_exit_code_is_a_failure_rather_than_a_pass() {
    // Arrange
    let runner = FakeCommandRunner::new().with_streaming_code(None);

    // Act
    let result = gate(&runner).run();

    // Assert
    assert!(result.is_err());
}
