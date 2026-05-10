// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleStats {
    pub path: String,
    pub function_count: usize,
    pub avg_cyclomatic: f64,
    pub max_cyclomatic: u32,
    pub total_cyclomatic: u32,
    pub avg_braintax: f64,
    pub max_braintax: f64,
}
