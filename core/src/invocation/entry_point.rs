// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::ffi::OsString;
use std::process::ExitCode;

use anyhow::Result;

use crate::analysis::fs_walk::FsWalk;
use crate::invocation::app::App;
use crate::invocation::args::Args;
use crate::invocation::config::Config;
use crate::reporting::default_scorer::DefaultScorer;
use crate::reporting::stdout_reporter::StdoutReporter;

pub struct EntryPoint;

impl EntryPoint {
    #[must_use]
    pub fn compose(config: Config) -> App {
        App::new(
            Box::new(FsWalk::new(config.path())),
            Box::new(DefaultScorer::new()),
            Box::new(StdoutReporter::new(config.json(), config.top())),
            config,
        )
    }

    pub fn run(args: Vec<OsString>) -> ExitCode {
        match Self::run_from_args(Args::without_cargo_subcommand(args)) {
            Ok(exit_code) => exit_code,
            Err(error) => {
                eprintln!("error: {error}");
                ExitCode::FAILURE
            }
        }
    }

    pub fn run_from_args<I, T>(args: I) -> Result<ExitCode>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Self::compose(Config::from_args(Args::parse_from_args(args))).run()
    }
}
