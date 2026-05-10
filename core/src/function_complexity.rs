// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionComplexity {
    pub name: String,
    pub file: String,
    pub module: String,
    pub cyclomatic: u32,
    pub cfg_gates: u32,
    pub hidden_deps: u32,
    pub braintax: f64,
}
