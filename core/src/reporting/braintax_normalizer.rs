// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

const CEILING: f64 = 15.0;

#[derive(Debug, Clone, Default)]
pub struct BraintaxNormalizer;

impl BraintaxNormalizer {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    #[must_use]
    pub fn normalize(&self, braintax: f64) -> u32 {
        let ratio = (1.0 - braintax / CEILING).clamp(0.0, 1.0);
        (ratio * 100.0).round() as u32
    }
}
