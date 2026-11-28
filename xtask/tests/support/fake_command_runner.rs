// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serge4rust::process::command_outcome::CommandOutcome;
use serge4rust::process::command_runner::CommandRunner;
use serge4rust::process::launch_failure::LaunchFailure;
use std::cell::RefCell;

// The seam the self-analysis gate is constructed against: every program is
// available and starts, answers with the exit code it was given, and records
// the arguments it was called with.
pub struct FakeCommandRunner {
    exit_code: Option<i32>,
    calls: RefCell<Vec<Vec<String>>>,
}

impl FakeCommandRunner {
    pub fn new() -> Self {
        Self {
            exit_code: Some(0),
            calls: RefCell::new(Vec::new()),
        }
    }

    pub fn with_streaming_code(mut self, streaming_code: Option<i32>) -> Self {
        self.exit_code = streaming_code;
        self
    }

    pub fn calls(&self) -> Vec<Vec<String>> {
        self.calls.borrow().clone()
    }

    fn record(&self, args: &[String]) {
        self.calls.borrow_mut().push(args.to_vec());
    }
}

impl Default for FakeCommandRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandRunner for FakeCommandRunner {
    fn is_available(&self, _binary_name: &str) -> bool {
        true
    }

    fn run_streaming(&self, _program: &str, args: &[String]) -> Result<Option<i32>, LaunchFailure> {
        self.record(args);
        Ok(self.exit_code)
    }

    fn run_capturing(
        &self,
        _program: &str,
        args: &[String],
    ) -> Result<CommandOutcome, LaunchFailure> {
        self.record(args);
        Ok(CommandOutcome::new(
            self.exit_code,
            String::new(),
            String::new(),
        ))
    }
}
