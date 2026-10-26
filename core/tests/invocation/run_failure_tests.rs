// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_source_tree::NotHeld;
use braintax::analysis::source_path::SourcePath;
use braintax::analysis::source_read_failure::SourceReadFailure;
use braintax::invocation::run_failure::RunFailure;
use braintax::reporting::render_failure::RenderFailure;
use std::error::Error;

#[test]
fn fmt_of_a_render_failure_speaks_for_it() {
    // Arrange
    let failure = RunFailure::from(RenderFailure {
        cause: Box::new(NotHeld),
    });

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "failed to render the report");
}

#[test]
fn fmt_of_a_source_read_failure_speaks_for_it() {
    // Arrange
    let failure = RunFailure::from(SourceReadFailure {
        path: SourcePath::new(String::from("src/lib.rs")),
        cause: Box::new(NotHeld),
    });

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "failed to read src/lib.rs");
}

#[test]
fn fmt_of_no_source_files_names_the_root() {
    // Arrange
    let failure = RunFailure::NoSourceFiles(SourcePath::new(String::from("core")));

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "no Rust source files found in core");
}

#[test]
fn source_of_a_render_failure_is_its_cause() {
    // Arrange
    let failure = RunFailure::from(RenderFailure {
        cause: Box::new(NotHeld),
    });

    // Act
    let source = failure.source().map(ToString::to_string);

    // Assert
    assert_eq!(source.as_deref(), Some("not held"));
}

#[test]
fn source_of_a_source_read_failure_is_its_cause() {
    // Arrange
    let failure = RunFailure::from(SourceReadFailure {
        path: SourcePath::new(String::from("src/lib.rs")),
        cause: Box::new(NotHeld),
    });

    // Act
    let source = failure.source().map(ToString::to_string);

    // Assert
    assert_eq!(source.as_deref(), Some("not held"));
}

#[test]
fn source_of_no_source_files_is_none() {
    // Arrange
    let failure = RunFailure::NoSourceFiles(SourcePath::new(String::from("core")));

    // Act
    let source = failure.source();

    // Assert
    assert!(source.is_none());
}
