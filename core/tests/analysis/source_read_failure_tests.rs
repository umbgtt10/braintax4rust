// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_source_tree::NotHeld;
use braintax::analysis::source_path::SourcePath;
use braintax::analysis::source_read_failure::SourceReadFailure;
use std::error::Error;

#[test]
fn fmt_names_the_file_it_could_not_read() {
    // Arrange
    let failure = SourceReadFailure {
        path: SourcePath::new(String::from("src/lib.rs")),
        cause: Box::new(NotHeld),
    };

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "failed to read src/lib.rs");
}

#[test]
fn source_is_the_cause_it_carries() {
    // Arrange
    let failure = SourceReadFailure {
        path: SourcePath::new(String::from("src/lib.rs")),
        cause: Box::new(NotHeld),
    };

    // Act
    let source = failure.source().map(ToString::to_string);

    // Assert
    assert_eq!(source.as_deref(), Some("not held"));
}
