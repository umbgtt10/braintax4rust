// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use braintax::analysis::source_path::SourcePath;
use braintax::analysis::source_read_failure::SourceReadFailure;
use braintax::traits::source_tree::SourceTree;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

// The source tree every core test reads, held in memory: a path either holds
// its contents or is listed and cannot be read. Listed in path order. No test
// of the core touches a disk.
pub struct InMemorySourceTree {
    files: BTreeMap<String, Option<String>>,
}

impl InMemorySourceTree {
    pub fn new() -> Self {
        Self {
            files: BTreeMap::new(),
        }
    }

    pub fn with_file(mut self, path: &str, contents: &str) -> Self {
        self.files
            .insert(String::from(path), Some(String::from(contents)));
        self
    }

    pub fn with_unreadable_file(mut self, path: &str) -> Self {
        self.files.insert(String::from(path), None);
        self
    }
}

impl Default for InMemorySourceTree {
    fn default() -> Self {
        Self::new()
    }
}

impl SourceTree for InMemorySourceTree {
    fn read_source(&self, file: &SourcePath) -> Result<String, SourceReadFailure> {
        self.files
            .get(file.as_str())
            .cloned()
            .flatten()
            .ok_or_else(|| SourceReadFailure {
                path: file.clone(),
                cause: Box::new(NotHeld),
            })
    }

    fn rust_files(&self, root: &SourcePath) -> Result<Vec<SourcePath>, SourceReadFailure> {
        Ok(self
            .files
            .keys()
            .map(|path| SourcePath::new(path.clone()))
            .filter(|path| path.strip_prefix(root).is_some())
            .filter(|path| path.as_str().ends_with(".rs"))
            .collect())
    }
}

#[derive(Debug)]
pub struct NotHeld;

impl fmt::Display for NotHeld {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("not held")
    }
}

impl Error for NotHeld {}
