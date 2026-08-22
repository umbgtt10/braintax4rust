// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// A reporter that keeps what it rendered, so a test can read the document the
// tool would have printed instead of scraping stdout.
//
// It is test support rather than shipped code, which is exactly the sort of
// thing that acquires a public constructor nobody calls: `new` existed and
// every caller went through `default`.

use braintax_test_utils::capture_reporter::CaptureReporter;

#[test]
fn new_agrees_with_default() {
    // Arrange & Act
    let from_new = CaptureReporter::new();
    let from_default = CaptureReporter::default();

    // Assert
    assert_eq!(
        *from_new.captured.lock().expect("the lock holds"),
        *from_default.captured.lock().expect("the lock holds")
    );
}

#[test]
fn new_starts_with_nothing_captured() {
    // Arrange & Act
    let reporter = CaptureReporter::new();

    // Assert
    assert!(reporter.captured.lock().expect("the lock holds").is_empty());
}
