// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::analysis::collector::Collector;
use crate::counting::function_complexity::FunctionComplexity;
use crate::invocation::config::Config;
use crate::invocation::run_failure::RunFailure;
use crate::invocation::run_outcome::RunOutcome;
use crate::reporting::braintax_report::BraintaxReport;
use crate::reporting::threshold_gate::ThresholdGate;
use crate::traits::report_renderer::ReportRenderer;
use crate::traits::scorer::Scorer;
use crate::traits::walk::Walk;

pub struct App {
    walker: Box<dyn Walk>,
    scorer: Box<dyn Scorer>,
    renderer: Box<dyn ReportRenderer>,
    gate: ThresholdGate,
    config: Config,
}

impl App {
    #[must_use]
    pub fn new(
        walker: Box<dyn Walk>,
        scorer: Box<dyn Scorer>,
        renderer: Box<dyn ReportRenderer>,
        config: Config,
    ) -> Self {
        Self {
            walker,
            scorer,
            renderer,
            gate: ThresholdGate::new(config.threshold, config.max_avg_braintax),
            config,
        }
    }

    // The report, the text it renders to and the verdict on it. Printing the
    // text and turning the verdict into an exit code are the caller's.
    pub fn run(&self) -> Result<RunOutcome, RunFailure> {
        let functions = self.collect_files()?;

        if functions.is_empty() {
            return Err(RunFailure::NoSourceFiles(self.config.path.clone()));
        }

        let report = self.compute_report(functions);
        let output = self.renderer.render(&report)?;
        let passes = self.gate.passes(&report.overall);
        Ok(RunOutcome {
            report,
            output,
            passes,
        })
    }

    fn collect_files(&self) -> Result<Vec<FunctionComplexity>, RunFailure> {
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
        let target = self.config.path.file_name().unwrap_or(".").to_string();

        BraintaxReport {
            version: String::from(env!("CARGO_PKG_VERSION")),
            target,
            overall,
            modules: module_stats,
            functions,
        }
    }
}
