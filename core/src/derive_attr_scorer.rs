// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::Attribute;

pub struct DeriveAttrScorer {
    known_derives: Vec<&'static str>,
}

impl Default for DeriveAttrScorer {
    fn default() -> Self {
        Self::new()
    }
}

impl DeriveAttrScorer {
    #[must_use]
    pub fn new() -> Self {
        Self {
            known_derives: vec![
                "Debug",
                "Clone",
                "Copy",
                "Default",
                "Eq",
                "PartialEq",
                "Ord",
                "PartialOrd",
                "Hash",
            ],
        }
    }

    #[must_use]
    pub fn score(&self, attr: &Attribute) -> u32 {
        let Ok(meta_list) = attr.meta.require_list() else {
            return 0;
        };
        let s = format!("{}", meta_list.tokens);
        let mut total = 0;
        for derive_name in s.trim_start_matches('(').trim_end_matches(')').split(',') {
            let cleaned = derive_name.trim();
            if !cleaned.is_empty() && !self.known_derives.contains(&cleaned) {
                total += 3;
            }
        }
        total
    }
}
