// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::GenericParam;

pub struct GenericsCounter;

impl GenericsCounter {
    pub fn score_generics(
        params: &syn::punctuated::Punctuated<GenericParam, syn::Token![,]>,
        where_clause: &Option<syn::WhereClause>,
    ) -> u32 {
        let mut total = 0;
        for param in params {
            match param {
                GenericParam::Type(type_param) => {
                    total += 2;
                    total += type_param.bounds.len() as u32;
                }
                GenericParam::Lifetime(_) => {}
                GenericParam::Const(_) => {
                    total += 3;
                }
            }
        }
        if let Some(clause) = where_clause {
            for predicate in &clause.predicates {
                if let syn::WherePredicate::Type(pred_type) = predicate {
                    total += pred_type.bounds.len() as u32;
                }
            }
        }
        total
    }
}
