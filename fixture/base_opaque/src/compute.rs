// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

fn f(a: i32) -> i32 {
    let mut b = 0;
    for c in 0..10 {
        if c % 2 == 0 {
            if c % 3 == 0 {
                b += c * 2;
            } else if c % 3 == 1 {
                b += c;
            } else {
                b -= c;
            }
        }
        match c {
            0 | 1 => b += 1,
            2 | 3 => b += 2,
            4 | 5 => b += 3,
            _ => {
                if b > 0 {
                    return b;
                }
            }
        }
        if c > 5 && c < 8 {
            loop {
                b += 1;
                if b > 20 {
                    break;
                }
                if b == 15 {
                    continue;
                }
            }
        }
    }
    b
}
