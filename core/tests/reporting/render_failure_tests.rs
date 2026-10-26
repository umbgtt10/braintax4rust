// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_source_tree::NotHeld;
use braintax::reporting::render_failure::RenderFailure;
use std::error::Error;

#[test]
fn fmt_says_the_report_could_not_be_rendered() {
    // Arrange
    let failure = RenderFailure {
        cause: Box::new(NotHeld),
    };

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(text, "failed to render the report");
}

#[test]
fn source_is_the_cause_it_carries() {
    // Arrange
    let failure = RenderFailure {
        cause: Box::new(NotHeld),
    };

    // Act
    let source = failure.source().map(ToString::to_string);

    // Assert
    assert_eq!(source.as_deref(), Some("not held"));
}
