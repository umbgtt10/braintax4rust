// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use core::fmt;
use core::ops::Range;

// A path held as text, because the core has no `std::path`. `/` is its only
// separator: the edge that hands paths in turns the platform's own into it. Its
// components are read the way `std::path` reads them -- an empty one and a `.`
// are skipped, except a `.` that opens a relative path -- so a file's name and
// its place under a root come out as they always have.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourcePath {
    text: String,
}

impl SourcePath {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    pub fn file_name(&self) -> Option<&str> {
        self.components()
            .last()
            .map(|(name, _)| *name)
            .filter(|name| !matches!(*name, "." | ".."))
    }

    pub fn strip_prefix(&self, base: &Self) -> Option<&str> {
        let own = self.components();
        let wanted = base.components();
        let matches = self.has_root() == base.has_root()
            && wanted.len() <= own.len()
            && own
                .iter()
                .zip(&wanted)
                .all(|((name, _), (expected, _))| name == expected);
        if !matches {
            return None;
        }
        Some(match (own.get(wanted.len()), own.last()) {
            (Some((_, first)), Some((_, last))) => &self.text[first.start..last.end],
            _ => "",
        })
    }

    fn components(&self) -> Vec<(&str, Range<usize>)> {
        let mut components = Vec::new();
        let mut start = 0;
        for name in self.text.split('/') {
            let end = start + name.len();
            if !name.is_empty() && (name != "." || start == 0) {
                components.push((name, start..end));
            }
            start = end + 1;
        }
        components
    }

    fn has_root(&self) -> bool {
        self.text.starts_with('/')
    }
}

impl fmt::Display for SourcePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.text)
    }
}
