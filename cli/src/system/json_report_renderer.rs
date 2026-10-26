// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::counting::function_complexity::FunctionComplexity;
use braintax::reporting::braintax_report::BraintaxReport;
use braintax::reporting::module_stats::ModuleStats;
use braintax::reporting::overall_stats::OverallStats;
use braintax::reporting::render_failure::RenderFailure;
use braintax::traits::report_renderer::ReportRenderer;
use serde::Serialize;
use serde_json::to_string_pretty;

// The machine surface. Its records are its own: the core's carry no serde, so
// the shape of the JSON is written down here, field for field in the order the
// document has always had.
#[derive(Serialize)]
struct JsonReport<'a> {
    pub version: &'a str,
    pub target: &'a str,
    pub overall: JsonOverall,
    pub modules: Vec<JsonModule<'a>>,
    pub functions: Vec<JsonFunction<'a>>,
}

#[derive(Serialize)]
struct JsonOverall {
    pub total_functions: usize,
    pub avg_cyclomatic: f64,
    pub max_cyclomatic: u32,
    pub total_cyclomatic: u32,
    pub avg_braintax: f64,
    pub max_braintax: f64,
    pub total_braintax: f64,
    pub braintax_normalized: u32,
}

#[derive(Serialize)]
struct JsonModule<'a> {
    pub path: &'a str,
    pub function_count: usize,
    pub avg_cyclomatic: f64,
    pub max_cyclomatic: u32,
    pub total_cyclomatic: u32,
    pub avg_braintax: f64,
    pub max_braintax: f64,
    pub total_braintax: f64,
    pub braintax_normalized: u32,
}

#[derive(Serialize)]
struct JsonFunction<'a> {
    pub name: &'a str,
    pub file: &'a str,
    pub module: &'a str,
    pub cyclomatic: u32,
    pub cfg_gates: u32,
    pub hidden_deps: u32,
    pub hidden_dep_weight: f64,
    pub hidden_dep_labels: &'a [String],
    pub depth: u32,
    pub trait_factor: f64,
    pub braintax: f64,
    pub braintax_normalized: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct JsonReportRenderer;

impl JsonReportRenderer {
    pub fn new() -> Self {
        Self
    }

    fn function(function: &FunctionComplexity) -> JsonFunction<'_> {
        JsonFunction {
            name: &function.name,
            file: &function.file,
            module: &function.module,
            cyclomatic: function.cyclomatic,
            cfg_gates: function.cfg_gates,
            hidden_deps: function.hidden_deps,
            hidden_dep_weight: function.hidden_dep_weight,
            hidden_dep_labels: &function.hidden_dep_labels,
            depth: function.depth,
            trait_factor: function.trait_factor,
            braintax: function.braintax,
            braintax_normalized: function.braintax_normalized,
        }
    }

    fn module(module: &ModuleStats) -> JsonModule<'_> {
        JsonModule {
            path: &module.path,
            function_count: module.function_count,
            avg_cyclomatic: module.avg_cyclomatic,
            max_cyclomatic: module.max_cyclomatic,
            total_cyclomatic: module.total_cyclomatic,
            avg_braintax: module.avg_braintax,
            max_braintax: module.max_braintax,
            total_braintax: module.total_braintax,
            braintax_normalized: module.braintax_normalized,
        }
    }

    fn overall(overall: &OverallStats) -> JsonOverall {
        JsonOverall {
            total_functions: overall.total_functions,
            avg_cyclomatic: overall.avg_cyclomatic,
            max_cyclomatic: overall.max_cyclomatic,
            total_cyclomatic: overall.total_cyclomatic,
            avg_braintax: overall.avg_braintax,
            max_braintax: overall.max_braintax,
            total_braintax: overall.total_braintax,
            braintax_normalized: overall.braintax_normalized,
        }
    }
}

impl ReportRenderer for JsonReportRenderer {
    fn render(&self, report: &BraintaxReport) -> Result<String, RenderFailure> {
        let document = JsonReport {
            version: &report.version,
            target: &report.target,
            overall: Self::overall(&report.overall),
            modules: report.modules.iter().map(Self::module).collect(),
            functions: report.functions.iter().map(Self::function).collect(),
        };
        to_string_pretty(&document).map_err(|error| RenderFailure {
            cause: Box::new(error),
        })
    }
}
