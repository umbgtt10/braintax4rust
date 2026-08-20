// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use crate::invocation::args::Args;

#[derive(Debug, Clone)]
pub struct Config {
    pub path: PathBuf,
    pub json: bool,
    pub threshold: Option<u32>,
    pub max_avg_braintax: Option<f64>,
    pub top: usize,
}

impl Config {
    #[must_use]
    pub fn from_args(args: Args) -> Self {
        Self {
            path: args.path,
            json: args.json,
            threshold: args.threshold,
            max_avg_braintax: args.max_avg_braintax,
            top: args.top.max(1),
        }
    }
}
