// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::gates::gate::Gate;
use crate::process::command_runner::CommandRunner;

pub struct BraintaxSelfGate<'a> {
    runner: &'a dyn CommandRunner,
    package: String,
    target: String,
    ceiling: String,
}

impl<'a> BraintaxSelfGate<'a> {
    pub fn new(
        runner: &'a dyn CommandRunner,
        package: String,
        target: String,
        ceiling: String,
    ) -> Self {
        Self {
            runner,
            package,
            target,
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
        // Pointed at a target rather than left to default to the working
        // directory. The default swept in fixture/, whose crates are analysis
        // input written to score badly -- 79 of the 178 functions measured, at
        // 26 to 52 brain tax each. They drowned out the tool's own code and
        // carried a platform disagreement with them.
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
            self.target.clone(),
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
