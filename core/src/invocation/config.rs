// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;
use std::path::PathBuf;

use crate::invocation::args::Args;

#[derive(Debug, Clone)]
pub struct Config {
    path: PathBuf,
    json: bool,
    threshold: Option<u32>,
    max_avg_braintax: Option<f64>,
    top: usize,
}

impl Config {
    #[must_use]
    pub fn from_args(args: Args) -> Self {
        Self {
            path: args.path().to_path_buf(),
            json: args.json(),
            threshold: args.threshold(),
            max_avg_braintax: args.max_avg_braintax(),
            top: args.top().max(1),
        }
    }

    #[must_use]
    pub fn new(
        path: PathBuf,
        json: bool,
        threshold: Option<u32>,
        max_avg_braintax: Option<f64>,
        top: usize,
    ) -> Self {
        Self {
            path,
            json,
            threshold,
            max_avg_braintax,
            top,
        }
    }

    pub fn json(&self) -> bool {
        self.json
    }

    pub fn max_avg_braintax(&self) -> Option<f64> {
        self.max_avg_braintax
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn threshold(&self) -> Option<u32> {
        self.threshold
    }

    pub fn top(&self) -> usize {
        self.top
    }
}
