// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone)]
pub struct TraitInfo {
    methods: u32,
    assoc_types: u32,
    supertraits: u32,
    impl_count: u32,
}

impl TraitInfo {
    #[must_use]
    pub fn new(methods: u32, assoc_types: u32, supertraits: u32, impl_count: u32) -> Self {
        Self {
            methods,
            assoc_types,
            supertraits,
            impl_count,
        }
    }

    pub fn assoc_types(&self) -> u32 {
        self.assoc_types
    }

    pub fn count_impl(&mut self) {
        self.impl_count += 1;
    }

    pub fn impl_count(&self) -> u32 {
        self.impl_count
    }

    pub fn methods(&self) -> u32 {
        self.methods
    }

    pub fn supertraits(&self) -> u32 {
        self.supertraits
    }
}
