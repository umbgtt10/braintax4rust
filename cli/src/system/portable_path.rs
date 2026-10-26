// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::{MAIN_SEPARATOR, Path};

use braintax::analysis::source_path::SourcePath;

// Where a platform path becomes the core's: `/` is the core's only separator,
// so on Windows each `\` turns into one; on Unix `\` is an ordinary byte of a
// name and stays. A name that is not UTF-8 is carried lossily, as 0.13.1
// reported it.
pub struct PortablePath;

impl PortablePath {
    pub fn of(path: &Path) -> SourcePath {
        let text = path.to_string_lossy();
        if MAIN_SEPARATOR == '\\' {
            SourcePath::new(text.replace('\\', "/"))
        } else {
            SourcePath::new(text.into_owned())
        }
    }
}
