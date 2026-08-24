// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::gates::gate::Gate;
use crate::process::command_runner::CommandRunner;

pub struct BraintaxSelfGate<'a> {
    runner: &'a dyn CommandRunner,
    package: String,
    ceiling: String,
}

impl<'a> BraintaxSelfGate<'a> {
    pub fn new(runner: &'a dyn CommandRunner, package: String, ceiling: String) -> Self {
        Self {
            runner,
            package,
            ceiling,
        }
    }
}

impl Gate for BraintaxSelfGate<'_> {
    fn label(&self) -> String {
        String::from("braintax self-analysis")
    }

    fn run(&self) -> Result<(), String> {
        // Built from this checkout rather than taken from an install: a tool
        // that reports its own braintax has to report the tree being changed.
        //
        // The ceiling travels to the tool rather than being judged here, so
        // there is one implementation of the comparison and it is the tool's.
        // A non-zero exit is that verdict, not a failure to run.
        let args = vec![
            String::from("run"),
            String::from("--quiet"),
            String::from("--package"),
            self.package.clone(),
            String::from("--"),
            String::from("--max-avg-braintax"),
            self.ceiling.clone(),
        ];

        match self.runner.run_streaming("cargo", &args)? {
            Some(0) => Ok(()),
            _ => Err(format!(
                "avg braintax exceeds the ceiling of {}",
                self.ceiling
            )),
        }
    }
}
