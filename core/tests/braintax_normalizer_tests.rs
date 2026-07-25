// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::braintax_normalizer::BraintaxNormalizer;

#[test]
fn normalize_zero_braintax_returns_100() {
    // Arrange
    let normalizer = BraintaxNormalizer::new();

    // Act
    let normalized = normalizer.normalize(0.0);

    // Assert
    assert_eq!(normalized, 100);
}

#[test]
fn normalize_braintax_at_half_ceiling_returns_50() {
    // Arrange
    let normalizer = BraintaxNormalizer::new();

    // Act
    let normalized = normalizer.normalize(7.5);

    // Assert
    assert_eq!(normalized, 50);
}

#[test]
fn normalize_braintax_at_ceiling_returns_0() {
    // Arrange
    let normalizer = BraintaxNormalizer::new();

    // Act
    let normalized = normalizer.normalize(15.0);

    // Assert
    assert_eq!(normalized, 0);
}

#[test]
fn normalize_braintax_above_ceiling_clamps_to_0() {
    // Arrange
    let normalizer = BraintaxNormalizer::new();

    // Act
    let normalized = normalizer.normalize(30.0);

    // Assert
    assert_eq!(normalized, 0);
}
