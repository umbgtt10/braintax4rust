// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

pub mod app;
pub mod args;
pub mod braintax_report;
pub mod collector;
pub mod complexity_visitor;
pub mod config;
pub mod default_scorer;
pub mod fs_walk;
pub mod function_complexity;
pub mod hidden_deps_counter;
pub mod macro_counter;
pub mod module_stats;
pub mod name_opacity_counter;
pub mod overall_stats;
pub mod stdout_reporter;
pub mod traits;
