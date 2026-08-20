// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone)]
pub struct TraitInfo {
    pub methods: u32,
    pub assoc_types: u32,
    pub supertraits: u32,
    pub impl_count: u32,
}
