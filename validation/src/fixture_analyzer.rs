// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use anyhow::Result;
use braintax::invocation::config::Config;
use braintax::reporting::braintax_report::BraintaxReport;
use braintax_cli::invocation::entry_point::EntryPoint;
use braintax_cli::system::portable_path::PortablePath;
use std::path::PathBuf;

pub struct FixtureAnalyzer {
    fixture_root: PathBuf,
}

impl FixtureAnalyzer {
    pub fn new(fixture_root: PathBuf) -> Self {
        Self { fixture_root }
    }

    pub fn analyze(&self, name: &str) -> Result<BraintaxReport> {
        let path = PortablePath::of(&self.fixture_root.join(name));
        let outcome = EntryPoint::compose(Config {
            path,
            json: true,
            threshold: None,
            max_avg_braintax: None,
            top: 10,
        })
        .run()?;
        Ok(outcome.report)
    }
}
