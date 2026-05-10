// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::custom_add;

fn compute(value: i32) -> i32 {
    let result = 0;
    for index in 0..10 {
        let _sum = custom_add!(result, index);
        if index % 2 == 0 {
            let _ = custom_add!(result, 1);
        }
    }
    result
}
