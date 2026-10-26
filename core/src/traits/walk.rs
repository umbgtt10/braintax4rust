// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::vec::Vec;

use crate::analysis::source_path::SourcePath;
use crate::analysis::source_read_failure::SourceReadFailure;

pub trait Walk {
    fn rust_files(&self) -> Result<Vec<(SourcePath, String)>, SourceReadFailure>;
}
