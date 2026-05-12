// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

pub struct Processor;

impl std::fmt::Debug for Processor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
                        return write!(f, "{}", result);
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
        write!(f, "{}", result)
    }
}
