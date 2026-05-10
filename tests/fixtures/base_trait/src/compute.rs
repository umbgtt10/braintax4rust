// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
use crate::{Computer, MyComputer};
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

#[cfg(feature = "complex")]
fn compute(x: i32) -> i32 {
    let mut result = 0;
    for i in 0..10 {
        if i % 2 == 0 {
            if i % 3 == 0 {
                result += i * 2;
            } else if i % 3 == 1 {
                result += i;
            } else {
                result -= i;
            }
        }

        match i {
            0 | 1 => result += 1,
            2 | 3 => result += 2,
            4 | 5 => result += 3,
            _ => {
                if result > 0 {
                    return result;
                }
            }
        }

        if i > 5 && i < 8 {
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
