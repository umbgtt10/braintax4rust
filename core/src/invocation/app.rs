// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::ffi::OsString;
use std::process::ExitCode;

use anyhow::Result;

use crate::analysis::collector::Collector;
use crate::analysis::fs_walk::FsWalk;
use crate::counting::function_complexity::FunctionComplexity;
use crate::invocation::args::Args;
use crate::invocation::config::Config;
use crate::reporting::braintax_report::BraintaxReport;
use crate::reporting::default_scorer::DefaultScorer;
use crate::reporting::stdout_reporter::StdoutReporter;
use crate::reporting::threshold_gate::ThresholdGate;
use crate::traits::reporter::Reporter;
use crate::traits::scorer::Scorer;
use crate::traits::walk::Walk;

pub struct App {
    walker: Box<dyn Walk>,
    scorer: Box<dyn Scorer>,
    reporter: Box<dyn Reporter>,
    gate: ThresholdGate,
    config: Config,
}

impl App {
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self {
            walker: Box::new(FsWalk::new(&config.path)),
            scorer: Box::new(DefaultScorer::new()),
            reporter: Box::new(StdoutReporter::new(config.json, config.top)),
            gate: ThresholdGate::new(config.threshold, config.max_avg_braintax),
            config,
        }
    }

    #[must_use]
    pub fn with_deps(
        walker: Box<dyn Walk>,
        scorer: Box<dyn Scorer>,
        reporter: Box<dyn Reporter>,
        config: Config,
    ) -> Self {
        Self {
            walker,
            scorer,
            reporter,
            gate: ThresholdGate::new(config.threshold, config.max_avg_braintax),
            config,
        }
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
        let traits = Collector::build_trait_registry(&files);
        let mut all_functions = Vec::new();
        for (path, source) in &files {
            let functions = Collector::collect(source, path, &self.config.path, &traits);
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
        self.reporter.write(report)?;

        Ok(if self.gate.passes(&report.overall) {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        })
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
