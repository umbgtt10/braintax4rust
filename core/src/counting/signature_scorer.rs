// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::ToString;

use crate::analysis::trait_info::TraitInfo;
use syn::FnArg;
use syn::GenericArgument;
use syn::PathArguments;
use syn::Receiver;
use syn::ReceiverKind;
use syn::ReturnType;
use syn::Token;
use syn::Type;
use syn::TypeParamBound;
use syn::TypePath;
use syn::punctuated::Punctuated;

const KNOWN_TRAITS: &[&str] = &[
    "Debug",
    "Clone",
    "Copy",
    "Default",
    "Eq",
    "PartialEq",
    "Ord",
    "PartialOrd",
    "Hash",
    "Display",
    "Iterator",
    "Into",
    "From",
    "Send",
    "Sync",
    "Drop",
];

const MARKER_TRAITS: &[&str] = &["Send", "Sync", "Unpin", "Sized"];

const POINTER_TYPES: &[&str] = &["Arc", "Box", "Rc"];

pub struct SignatureScorer;

impl SignatureScorer {
    // Zero-cost auto-traits add no methods and no conceptual surface, so adding
    // `: Send` to satisfy a lint must not inflate the dimension penalty for
    // every implementor.
    #[must_use]
    pub fn is_marker_supertrait(bound: &TypeParamBound) -> bool {
        match bound {
            TypeParamBound::Lifetime(_) => true,
            TypeParamBound::Trait(trait_bound) => {
                trait_bound.path.segments.last().is_some_and(|segment| {
                    MARKER_TRAITS.contains(&segment.ident.to_string().as_str())
                })
            }
            _ => false,
        }
    }

    #[must_use]
    pub fn return_complexity(return_type: &ReturnType) -> f64 {
        match return_type {
            ReturnType::Default => 0.0,
            ReturnType::Type(_, ty) => Self::type_complexity(ty),
        }
    }

    #[must_use]
    pub fn self_ref_cost(inputs: &Punctuated<FnArg, Token![,]>) -> f64 {
        match inputs.first() {
            Some(FnArg::Receiver(receiver)) => {
                if Self::is_mutable_receiver(receiver) {
                    0.4
                } else {
                    0.2
                }
            }
            _ => 0.0,
        }
    }

    #[must_use]
    pub fn trait_factor(name: &str, info: &TraitInfo) -> f64 {
        if KNOWN_TRAITS.contains(&name) {
            return 0.80;
        }
        let method_penalty = (info.methods().saturating_sub(3)) as f64 * 0.01;
        let base = if info.assoc_types() > 0 || info.supertraits() > 0 {
            1.15 + method_penalty
        } else {
            0.90 + method_penalty
        };
        base + Self::dimension_penalty(info) + Self::dispatch_penalty(info)
    }

    fn dimension_penalty(info: &TraitInfo) -> f64 {
        let assoc_dim: u32 = (info.assoc_types() > 0).into();
        let supertrait_dim: u32 = (info.supertraits() > 0).into();
        let extra_dims = assoc_dim + supertrait_dim;
        match extra_dims {
            0 => 0.0,
            1 => 0.15,
            _ => 0.20,
        }
    }

    fn dispatch_penalty(info: &TraitInfo) -> f64 {
        let base = ((info.impl_count().saturating_sub(1)) as f64 * 0.06).min(0.18);
        let amplifier = if info.assoc_types() > 0 { 1.5 } else { 1.0 };
        base * amplifier
    }

    fn is_mutable_receiver(receiver: &Receiver) -> bool {
        receiver.mutability.is_some()
            || matches!(receiver.kind, ReceiverKind::Reference(_, _, Some(_)))
    }

    fn path_type_complexity(type_path: &TypePath) -> f64 {
        let mut cost = 0.0;
        if type_path.qself.is_some() {
            cost += 1.0;
        }
        if type_path
            .path
            .segments
            .first()
            .is_some_and(|segment| segment.ident == "Self")
        {
            cost += 1.0;
        }
        for segment in &type_path.path.segments {
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                cost += args.args.len() as f64 * 0.3;
            }
        }
        cost
    }

    fn pointee(type_path: &TypePath) -> Option<&Type> {
        let segment = type_path.path.segments.last()?;
        let PathArguments::AngleBracketed(args) = &segment.arguments else {
            return None;
        };
        match args.args.first() {
            Some(GenericArgument::Type(pointee))
                if POINTER_TYPES.contains(&segment.ident.to_string().as_str()) =>
            {
                Some(pointee)
            }
            _ => None,
        }
    }

    fn type_complexity(ty: &Type) -> f64 {
        match ty {
            Type::ImplTrait(_) => 1.5,
            Type::TraitObject(_) => 1.0,
            Type::Paren(paren) => Self::type_complexity(&paren.elem),
            Type::Reference(reference) => Self::type_complexity(&reference.elem),
            Type::Path(type_path) => Self::pointee(type_path).map_or_else(
                || Self::path_type_complexity(type_path),
                Self::type_complexity,
            ),
            _ => 0.0,
        }
    }
}
