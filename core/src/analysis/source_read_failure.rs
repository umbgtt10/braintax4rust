// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use core::error::Error;
use core::fmt;

use crate::analysis::source_path::SourcePath;

// A Rust file the source tree listed but could not hand over, and its reason.
#[derive(Debug)]
pub struct SourceReadFailure {
    pub path: SourcePath,
    pub cause: Box<dyn Error + Send + Sync>,
}

impl fmt::Display for SourceReadFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "failed to read {}", self.path)
    }
}

impl Error for SourceReadFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.cause.as_ref())
    }
}
