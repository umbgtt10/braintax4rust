// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

pub trait Base {
    type Output;
    type Error;
    fn process(&self) -> Result<Self::Output, Self::Error>;
    fn extra_one(&self);
    fn extra_two(&self);
    fn extra_three(&self);
}

pub trait Extended: Base {
    type Context;
    fn run(&self, ctx: Self::Context) -> Self::Output;
    fn cleanup(&self);
}

pub struct Engine;

impl Base for Engine {
    type Output = i32;
    type Error = String;
    fn process(&self) -> Result<i32, String> {
        Ok(42)
    }
    fn extra_one(&self) {}
    fn extra_two(&self) {}
    fn extra_three(&self) {}
}

impl Extended for Engine {
    type Context = i32;
    fn run(&self, ctx: i32) -> i32 {
        let mut result = ctx;
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
    fn cleanup(&self) {}
}
