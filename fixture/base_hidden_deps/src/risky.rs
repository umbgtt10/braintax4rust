// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::time::Instant;

fn risky(value: i32) -> i32 {
    unsafe {
        let pointer: *const i32 = std::ptr::null();
        let _ = pointer.read();
    }
    println!("computing");
    let _now = Instant::now();
    if value > 0 { value } else { -value }
}
