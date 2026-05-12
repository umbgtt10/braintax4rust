// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fmt::Debug;

fn compute<A, B, E>(value: i32, _a: A, _b: B, _e: E) -> i32
where
    A: Debug + Clone,
    B: Debug,
{
    let mut result = 0;
    for index in 0..10 {
        if index % 2 == 0 {
            if index % 3 == 0 {
                result += index * 2;
            } else if index % 3 == 1 {
                result += index;
            } else {
                result -= index;
            }
        }
        match index {
            0 | 1 => result += 1,
            2 | 3 => result += 2,
            4 | 5 => result += 3,
            _ => {
                if result > 0 {
                    return result;
                }
            }
        }
        if index > 5 && index < 8 {
            loop {
                result += 1;
                if result > 20 {
                    break;
                }
                if result == 15 {
                    continue;
                }
            }
        }
    }
    result
}
