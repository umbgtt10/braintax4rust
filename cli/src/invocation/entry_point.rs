// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::ffi::OsString;
use std::process::ExitCode;

use anyhow::Result;

use braintax::analysis::source_walk::SourceWalk;
use braintax::invocation::app::App;
use braintax::invocation::config::Config;
use braintax::reporting::default_scorer::DefaultScorer;
use braintax::reporting::stdout_reporter::StdoutReporter;

use crate::invocation::args::Args;
use crate::system::walkdir_source_tree::WalkdirSourceTree;

pub struct EntryPoint;

impl EntryPoint {
    #[must_use]
    pub fn compose(config: Config) -> App {
        App::new(
            Box::new(SourceWalk::new(
                Box::new(WalkdirSourceTree::new()),
                config.path.clone(),
            )),
            Box::new(DefaultScorer::new()),
            Box::new(StdoutReporter::new(config.json, config.top)),
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
        Self::compose(Args::parse_from_args(args).config()).run()
    }
}
