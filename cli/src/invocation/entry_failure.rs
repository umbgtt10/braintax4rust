// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::error::Error;
use std::fmt;
use std::io;

use braintax::invocation::run_failure::RunFailure;

// Why the command produced no verdict: the run failed, or its report could not
// be written to stdout. Either speaks for itself.
#[derive(Debug)]
pub enum EntryFailure {
    Run(RunFailure),
    Write(io::Error),
}

impl fmt::Display for EntryFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Run(failure) => failure.fmt(formatter),
            Self::Write(error) => error.fmt(formatter),
        }
    }
}

impl Error for EntryFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Run(failure) => failure.source(),
            Self::Write(error) => error.source(),
        }
    }
}

impl From<RunFailure> for EntryFailure {
    fn from(failure: RunFailure) -> Self {
        Self::Run(failure)
    }
}

impl From<io::Error> for EntryFailure {
    fn from(error: io::Error) -> Self {
        Self::Write(error)
    }
}
