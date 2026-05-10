// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io::{self, Write};

use anyhow::Result;

use crate::braintax_report::BraintaxReport;
use crate::function_complexity::FunctionComplexity;
use crate::module_stats::ModuleStats;
use crate::traits::reporter::Reporter;

#[derive(Debug, Clone)]
pub struct StdoutReporter {
    json: bool,
    top: usize,
}

impl StdoutReporter {
    #[must_use]
    pub fn new(json: bool, top: usize) -> Self {
        Self { json, top }
    }
}

impl Reporter for StdoutReporter {
    fn render(&self, report: &BraintaxReport) -> Result<String> {
        if self.json {
            Ok(serde_json::to_string_pretty(report)?)
        } else {
            Ok(self.render_human(report))
        }
    }

    fn write(&self, report: &BraintaxReport) -> Result<()> {
        let out = self.render(report)?;
        io::stdout().write_all(out.as_bytes())?;
        io::stdout().write_all(b"\n")?;
        Ok(())
    }
}

impl StdoutReporter {
    fn render_human(&self, report: &BraintaxReport) -> String {
        let mut lines = Vec::new();
        let target = &report.target;
        lines.push(format!(
            "cargo-braintax4rust {} -- {}\n══════════════════════════════════════════════",
            report.version, target,
        ));

        let overall = &report.overall;
        lines.push(String::new());
        lines.push(format!(
            "  Overall braintax:            {:.1}",
            overall.avg_braintax
        ));
        lines.push(format!(
            "  Maximum braintax:           {:.1}",
            overall.max_braintax
        ));
        lines.push(String::new());
        lines.push("Cyclomatic complexity:".to_string());
        lines.push(format!(
            "  Total functions:             {}",
            overall.total_functions
        ));
        lines.push(format!(
            "  Average complexity:         {:.1}",
            overall.avg_cyclomatic
        ));
        lines.push(format!(
            "  Maximum complexity:         {}",
            overall.max_cyclomatic
        ));
        lines.push(format!(
            "  Total complexity:           {}",
            overall.total_cyclomatic
        ));

        if !report.modules.is_empty() {
            lines.push("\nPer module:".to_string());
            lines.push(format!(
                "  {:<30}  {:<6}  {:<8}  {:<5}",
                "Module", "Funcs", "Avg BT", "Max"
            ));
            lines.push(format!("  {:-<30}  {:-<6}  {:-<8}  {:-<5}", "", "", "", ""));
            for module in &report.modules {
                lines.push(self.render_module_line(module));
            }
        }

        let top_n = self.top_functions(&report.functions);
        if !top_n.is_empty() {
            lines.push(format!(
                "\nTop {} most complex functions:",
                self.top.min(top_n.len())
            ));
            lines.push(format!(
                "  {:<50}  {:<12}  {:>5}  {:>6}",
                "Function", "Module", "CC", "BT"
            ));
            lines.push(format!(
                "  {:-<50}  {:-<12}  {:-<5}  {:-<6}",
                "", "", "", ""
            ));
            for func in &top_n {
                let location = if func.module == "." {
                    func.file.clone()
                } else {
                    format!("{}::{}", func.module, func.name)
                };
                lines.push(format!(
                    "  {:<50}  {:<12}  {:>5}  {:>6.1}",
                    location, func.module, func.cyclomatic, func.braintax
                ));
            }
        }

        lines.join("\n")
    }

    fn render_module_line(&self, module: &ModuleStats) -> String {
        format!(
            "  {:<30}  {:<6}  {:<8.1}  {:<5}",
            module.path, module.function_count, module.avg_braintax, module.max_braintax,
        )
    }

    fn top_functions<'a>(
        &self,
        functions: &'a [FunctionComplexity],
    ) -> Vec<&'a FunctionComplexity> {
        let mut sorted: Vec<&FunctionComplexity> = functions.iter().collect();
        sorted.sort_by(|a, b| b.cyclomatic.cmp(&a.cyclomatic));
        sorted.truncate(self.top);
        sorted
    }
}
