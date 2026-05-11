// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fmt::Debug;

fn transform<A, B, E>(input: Result<A, E>, mapper: fn(A) -> B) -> Option<B>
where
    A: Debug + Clone,
    B: Debug,
{
    match input {
        Ok(value) => Some(mapper(value)),
        Err(_) => None,
    }
}
