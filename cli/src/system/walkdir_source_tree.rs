// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs;
use std::path::Path;

use braintax::analysis::source_path::SourcePath;
use braintax::analysis::source_read_failure::SourceReadFailure;
use braintax::traits::source_tree::SourceTree;
use walkdir::WalkDir;

use crate::system::portable_path::PortablePath;

// The source tree on disk. An entry the walk cannot read is passed over, and a
// root that does not exist holds no files, as 0.13.1 had it.
#[derive(Debug, Clone, Copy, Default)]
pub struct WalkdirSourceTree;

impl WalkdirSourceTree {
    pub fn new() -> Self {
        Self
    }
}

impl SourceTree for WalkdirSourceTree {
    fn read_source(&self, file: &SourcePath) -> Result<String, SourceReadFailure> {
        fs::read_to_string(Path::new(file.as_str())).map_err(|error| SourceReadFailure {
            path: file.clone(),
            cause: Box::new(error),
        })
    }

    fn rust_files(&self, root: &SourcePath) -> Result<Vec<SourcePath>, SourceReadFailure> {
        Ok(WalkDir::new(root.as_str())
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| entry.path().extension() == Some("rs".as_ref()))
            .map(|entry| PortablePath::of(entry.path()))
            .collect())
    }
}
