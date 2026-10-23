// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// Mirrors src/analysis/trait_info.rs.

use braintax::analysis::trait_info::TraitInfo;

#[test]
fn count_impl_adds_one_implementation() {
    // Arrange
    let mut info = TraitInfo::new(2, 1, 0, 3);

    // Act
    info.count_impl();

    // Assert
    assert_eq!(info.impl_count(), 4);
}

#[test]
fn new_keeps_every_field_it_was_given() {
    // Arrange & Act
    let info = TraitInfo::new(2, 1, 0, 3);

    // Assert
    assert_eq!(info.methods(), 2);
    assert_eq!(info.assoc_types(), 1);
    assert_eq!(info.supertraits(), 0);
    assert_eq!(info.impl_count(), 3);
}
