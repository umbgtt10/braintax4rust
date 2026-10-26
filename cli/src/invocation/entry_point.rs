// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::ffi::OsString;
use std::io::{Write, stdout};
use std::process::ExitCode;

use anyhow::Result;

use braintax::analysis::source_walk::SourceWalk;
use braintax::invocation::app::App;
use braintax::invocation::config::Config;
use braintax::reporting::default_scorer::DefaultScorer;
use braintax::reporting::text_report_renderer::TextReportRenderer;
use braintax::traits::report_renderer::ReportRenderer;

use crate::invocation::args::Args;
use crate::system::json_report_renderer::JsonReportRenderer;
use crate::system::walkdir_source_tree::WalkdirSourceTree;

// The composition root: the command line in, the report out on stdout, an exit
// code back. The one place the file system, the report's format and stdout are
// chosen; everything it composes is the core's, tested without them.
pub struct EntryPoint;

impl EntryPoint {
    #[must_use]
    pub fn compose(config: Config) -> App {
        let renderer: Box<dyn ReportRenderer> = if config.json {
            Box::new(JsonReportRenderer::new())
        } else {
            Box::new(TextReportRenderer::new(config.top))
        };
        App::new(
            Box::new(SourceWalk::new(
                Box::new(WalkdirSourceTree::new()),
                config.path.clone(),
            )),
            Box::new(DefaultScorer::new()),
            renderer,
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
        let outcome = Self::compose(Args::parse_from_args(args).config()).run()?;
        stdout().write_all(outcome.output.as_bytes())?;
        stdout().write_all(b"\n")?;

        Ok(if outcome.passes {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        })
    }
}
