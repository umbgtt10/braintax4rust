// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::analysis::source_path::SourcePath;
use crate::analysis::source_read_failure::SourceReadFailure;
use crate::traits::source_tree::SourceTree;
use crate::traits::walk::Walk;

const BUILD_SCRIPT: &str = "build.rs";
const EXCLUDED_DIRECTORIES: [&str; 5] = ["target", ".git", "tests", "examples", "benches"];

// The Rust files under a root, each with its source: everything the source tree
// lists but a build script and whatever sits below a build, VCS, test, example
// or bench directory, which is never read.
pub struct SourceWalk {
    source_tree: Box<dyn SourceTree>,
    root: SourcePath,
}

impl SourceWalk {
    pub fn new(source_tree: Box<dyn SourceTree>, root: SourcePath) -> Self {
        Self { source_tree, root }
    }

    fn is_excluded(&self, path: &SourcePath) -> bool {
        if path.file_name() == Some(BUILD_SCRIPT) {
            return true;
        }
        path.strip_prefix(&self.root).is_some_and(|relative| {
            relative
                .split('/')
                .any(|name| EXCLUDED_DIRECTORIES.contains(&name))
        })
    }
}

impl Walk for SourceWalk {
    fn rust_files(&self) -> Result<Vec<(SourcePath, String)>, SourceReadFailure> {
        let mut files = Vec::new();
        for path in self.source_tree.rust_files(&self.root)? {
            if self.is_excluded(&path) {
                continue;
            }
            let source = self.source_tree.read_source(&path)?;
            files.push((path, source));
        }
        Ok(files)
    }
}
