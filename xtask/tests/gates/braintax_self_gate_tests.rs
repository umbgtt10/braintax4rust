// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::fake_command_runner::FakeCommandRunner;
use serge4rust::gates::gate::Gate;
use xtask::gates::braintax_self_gate::BraintaxSelfGate;

fn gate(runner: &FakeCommandRunner) -> BraintaxSelfGate<'_> {
    BraintaxSelfGate::new(
        runner,
        String::from("cargo-braintax4rust"),
        String::from("core"),
        String::from("5.0"),
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

// fixture/ holds crates written to score badly -- they are what braintax is
// pointed at, never its own code. Leaving the target to default swept them in
// and they outvoted core entirely.
#[test]
fn run_measures_the_configured_target_rather_than_the_whole_tree() {
    // Arrange
    let runner = FakeCommandRunner::new();

    // Act
    let _ = gate(&runner).run();

    // Assert
    let call = &runner.calls()[0];
    assert!(call.contains(&String::from("core")));
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
    assert!(call.contains(&String::from("5.0")));
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

// cargo's own exit code for a build that failed or a binary that panicked: the
// tool never gave a verdict, so no ceiling was breached.
#[test]
fn run_with_exit_code_101_says_it_could_not_run() {
    // Arrange
    let runner = FakeCommandRunner::new().with_streaming_code(Some(101));

    // Act
    let result = gate(&runner).run();

    // Assert
    assert_eq!(
        result.map_err(|failure| failure.to_string()),
        Err(String::from("could not run, exit code 101"))
    );
}

// 1 is the exit code cargo-braintax4rust documents for a breached bound.
#[test]
fn run_with_exit_code_1_names_the_ceiling_it_breached() {
    // Arrange
    let runner = FakeCommandRunner::new().with_streaming_code(Some(1));

    // Act
    let result = gate(&runner).run();

    // Assert
    assert_eq!(
        result.map_err(|failure| failure.to_string()),
        Err(String::from("avg braintax exceeds the ceiling of 5.0"))
    );
}

// 2 is cargo-braintax4rust's code for an error of its own and clap's for a
// usage error: either way the tool gave no verdict, so no ceiling was breached.
#[test]
fn run_with_exit_code_2_says_it_could_not_run() {
    // Arrange
    let runner = FakeCommandRunner::new().with_streaming_code(Some(2));

    // Act
    let result = gate(&runner).run();

    // Assert
    assert_eq!(
        result.map_err(|failure| failure.to_string()),
        Err(String::from("could not run, exit code 2"))
    );
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

#[test]
fn run_with_no_exit_code_says_a_signal_terminated_it() {
    // Arrange
    let runner = FakeCommandRunner::new().with_streaming_code(None);

    // Act
    let result = gate(&runner).run();

    // Assert
    assert_eq!(
        result.map_err(|failure| failure.to_string()),
        Err(String::from("could not run, terminated by signal"))
    );
}
