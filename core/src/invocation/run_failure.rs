// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use core::error::Error;
use core::fmt;

use crate::analysis::source_path::SourcePath;
use crate::analysis::source_read_failure::SourceReadFailure;
use crate::reporting::render_failure::RenderFailure;

// Why a run produced no report. A wrapped failure speaks for itself.
#[derive(Debug)]
pub enum RunFailure {
    NoSourceFiles(SourcePath),
    Render(RenderFailure),
    SourceRead(SourceReadFailure),
}

impl fmt::Display for RunFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSourceFiles(path) => write!(formatter, "no Rust source files found in {path}"),
            Self::Render(failure) => failure.fmt(formatter),
            Self::SourceRead(failure) => failure.fmt(formatter),
        }
    }
}

impl Error for RunFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NoSourceFiles(_) => None,
            Self::Render(failure) => failure.source(),
            Self::SourceRead(failure) => failure.source(),
        }
    }
}

impl From<RenderFailure> for RunFailure {
    fn from(failure: RenderFailure) -> Self {
        Self::Render(failure)
    }
}

impl From<SourceReadFailure> for RunFailure {
    fn from(failure: SourceReadFailure) -> Self {
        Self::SourceRead(failure)
    }
}
