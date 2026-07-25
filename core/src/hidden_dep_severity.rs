// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, Default)]
pub struct HiddenDepSeverity;

impl HiddenDepSeverity {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    #[must_use]
    pub fn severity(&self, label: &str) -> f64 {
        match label {
            "unsafe" => 8.0,
            "process::exit" | "process::abort" | "abort" => 6.0,
            "fs::read" | "fs::write" | "File::open" | "File::create" => 5.0,
            "Instant::now" | "SystemTime::now" | "random" | "rand::random" | "thread_rng"
            | "rand::thread_rng" => 4.0,
            "env::var" | "env::args" | "thread::sleep" => 3.0,
            "println" | "eprintln" | "print" | "eprint" => 2.0,
            _ => 4.0,
        }
    }
}
