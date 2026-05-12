// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

#![allow(dead_code, unused_variables, clippy::assign_op_pattern)]

pub mod compute;

#[macro_export]
macro_rules! custom_add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}
