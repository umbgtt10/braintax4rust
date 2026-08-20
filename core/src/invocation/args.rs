// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::ffi::OsString;
use std::path::PathBuf;

use clap::Parser;
use std::env::args_os;

#[derive(Debug, Clone, Parser)]
#[command(name = "cargo-braintax4rust")]
#[command(version = concat!("v", env!("CARGO_PKG_VERSION")))]
#[command(about = "Estimate the cognitive tax of Rust code")]
pub struct Args {
    #[arg(default_value = ".")]
    pub path: PathBuf,

    #[arg(long)]
    pub json: bool,

    #[arg(long, alias = "max-complexity")]
    pub threshold: Option<u32>,

    #[arg(long)]
    pub max_avg_braintax: Option<f64>,

    #[arg(long, default_value = "10")]
    pub top: usize,
}

impl Args {
    pub fn parse_from_args<I, T>(args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Self::parse_from(args)
    }

    // Cargo runs `cargo braintax4rust ...` as `cargo-braintax4rust
    // braintax4rust ...`, so the name arrives twice. Running the binary
    // directly does not repeat it, which is why the strip is conditional; and
    // it is positional, because dropping every occurrence would swallow a path
    // argument that happens to be named after the tool.
    pub fn without_cargo_subcommand(raw: Vec<OsString>) -> Vec<OsString> {
        if raw.len() > 1 && raw[1].to_string_lossy() == "braintax4rust" {
            let mut filtered = vec![raw[0].clone()];
            filtered.extend(raw.into_iter().skip(2));
            filtered
        } else {
            raw
        }
    }

    // Not public: reading the real process argv is glue, and the half of it
    // worth testing is above.
    pub(crate) fn parse_cargo() -> Self {
        Self::parse_from(Self::without_cargo_subcommand(args_os().collect()))
    }
}
