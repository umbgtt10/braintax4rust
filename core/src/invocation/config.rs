// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::analysis::source_path::SourcePath;

// What one run was asked for: the root to analyse, the report's format and
// length, and the two ceilings its verdict is held to.
#[derive(Debug, Clone)]
pub struct Config {
    pub path: SourcePath,
    pub json: bool,
    pub threshold: Option<u32>,
    pub max_avg_braintax: Option<f64>,
    pub top: usize,
}
