// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::ffi::OsString;
use std::path::PathBuf;

use braintax::invocation::config::Config;
use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(name = "cargo-braintax4rust")]
#[command(version = concat!("v", env!("CARGO_PKG_VERSION")))]
#[command(about = "Estimate the cognitive tax of Rust code")]
pub struct Args {
    #[arg(default_value = ".")]
    path: PathBuf,

    #[arg(long)]
    json: bool,

    #[arg(long, alias = "max-complexity")]
    threshold: Option<u32>,

    #[arg(long)]
    max_avg_braintax: Option<f64>,

    #[arg(long, default_value = "10")]
    top: usize,
}

impl Args {
    pub fn parse_from_args<I, T>(args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Self::parse_from(args)
    }

    // `--top 0` would show nothing, so it shows one.
    pub fn config(&self) -> Config {
        Config {
            path: self.path.clone(),
            json: self.json,
            threshold: self.threshold,
            max_avg_braintax: self.max_avg_braintax,
            top: self.top.max(1),
        }
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
}
