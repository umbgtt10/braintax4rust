// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::analysis::source_path::SourcePath;
use braintax::analysis::source_read_failure::SourceReadFailure;
use braintax::invocation::run_failure::RunFailure;
use braintax_cli::invocation::entry_failure::EntryFailure;
use std::error::Error as _;
use std::io::Error;
use std::io::ErrorKind;

fn read_failure() -> RunFailure {
    RunFailure::SourceRead(SourceReadFailure {
        path: SourcePath::new(String::from("src/lib.rs")),
        cause: Box::new(Error::new(ErrorKind::NotFound, "gone")),
    })
}

#[test]
fn fmt_of_a_run_failure_speaks_for_it() {
    // Arrange
    let failure = EntryFailure::from(read_failure());

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "failed to read src/lib.rs");
}

#[test]
fn fmt_of_a_write_failure_is_the_io_error() {
    // Arrange
    let failure = EntryFailure::from(Error::new(ErrorKind::BrokenPipe, "pipe closed"));

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "pipe closed");
}

#[test]
fn source_of_a_run_failure_is_its_cause() {
    // Arrange
    let failure = EntryFailure::from(read_failure());

    // Act
    let source = failure.source().map(ToString::to_string);

    // Assert
    assert_eq!(source.as_deref(), Some("gone"));
}

#[test]
fn source_of_a_write_failure_is_the_io_errors_own() {
    // Arrange
    let failure = EntryFailure::from(Error::new(ErrorKind::BrokenPipe, "pipe closed"));

    // Act
    let source = failure.source();

    // Assert
    assert!(source.is_none());
}
