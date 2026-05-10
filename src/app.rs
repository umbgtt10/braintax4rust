// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::ffi::OsString;
use std::path::Path;
use std::process::ExitCode;

use anyhow::Result;

use crate::args::Args;
use crate::braintax_report::BraintaxReport;
use crate::collector::Collector;
use crate::config::Config;
use crate::function_complexity::FunctionComplexity;

use crate::traits::reporter::Reporter;
use crate::traits::scorer::Scorer;
use crate::traits::walk::Walk;

#[derive(Debug)]
pub struct App<W: Walk, S: Scorer, R: Reporter> {
    walker: W,
    scorer: S,
    reporter: R,
    config: Config,
}

impl
    App<
        crate::fs_walk::FsWalk,
        crate::default_scorer::DefaultScorer,
        crate::stdout_reporter::StdoutReporter,
    >
{
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self {
            walker: crate::fs_walk::FsWalk::new(&config.path),
            scorer: crate::default_scorer::DefaultScorer::new(),
            reporter: crate::stdout_reporter::StdoutReporter::new(config.json, config.top),
            config,
        }
    }
}

impl<W: Walk, S: Scorer, R: Reporter> App<W, S, R> {
    #[must_use]
    pub fn with_deps(walker: W, scorer: S, reporter: R, config: Config) -> Self {
        Self {
            walker,
            scorer,
            reporter,
            config,
        }
    }

    #[must_use]
    pub fn reporter(&self) -> &R {
        &self.reporter
    }

    pub fn run(&self) -> Result<ExitCode> {
        let functions = self.collect_files()?;

        if functions.is_empty() {
            return Err(anyhow::anyhow!(
                "no Rust source files found in {}",
                self.config.path.display()
            ));
        }

        let report = self.compute_report(functions);
        self.handle_output(&report)
    }

    fn collect_files(&self) -> Result<Vec<FunctionComplexity>> {
        let files = self.walker.rust_files()?;
        let mut all_functions = Vec::new();
        for (path, source) in files {
            let functions = Collector::collect(&source, &path, &self.config.path);
            all_functions.extend(functions);
        }
        Ok(all_functions)
    }

    fn compute_report(&self, functions: Vec<FunctionComplexity>) -> BraintaxReport {
        let overall = self.scorer.overall_stats(&functions);
        let module_stats = self.scorer.module_stats(&functions);
        let target = self
            .config
            .path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(".")
            .to_string();

        BraintaxReport {
            version: env!("CARGO_PKG_VERSION").to_string(),
            target,
            overall,
            modules: module_stats,
            functions,
        }
    }

    fn handle_output(&self, report: &BraintaxReport) -> Result<ExitCode> {
        if let Some(max) = self.config.threshold {
            return Ok(if report.overall.max_cyclomatic <= max {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            });
        }
        self.reporter.write(report)?;
        Ok(ExitCode::SUCCESS)
    }

    #[allow(dead_code)]
    fn module_from_path(&self, path: &Path) -> String {
        let relative = path.strip_prefix(&self.config.path).unwrap_or(path);
        let s = relative.to_string_lossy().replace('\\', "/");
        let without_src = s.strip_prefix("src/").map(|s| s.to_string()).unwrap_or(s);
        if let Some(pos) = without_src.rfind('/') {
            without_src[..pos].to_string()
        } else {
            ".".to_string()
        }
    }
}

pub fn run() -> Result<ExitCode> {
    let args = Args::parse_cargo();
    let config = Config::from_args(args);
    App::new(config).run()
}

pub fn run_from_args<I, T>(args: I) -> Result<ExitCode>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let args = Args::parse_from_args(args);
    let config = Config::from_args(args);
    App::new(config).run()
}
