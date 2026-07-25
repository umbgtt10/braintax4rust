# ADR-AstOnlyNoTypeResolution

- **Status:** Accepted
- **Date:** 2026-07-25

## Context

`braintax` analyzes Rust source by parsing it with `syn::parse_file` into an
AST and walking it with `syn::visit::Visit`. It never performs type
resolution, borrow checking, or macro expansion — only syntactic pattern
matching over the parsed tree. Two consequences fall directly out of that
choice:

Trait-factor classification (`compute_trait_factor` in `collector.rs`)
decides whether an `impl`'s trait is a cheap, well-known one (flat `0.80`)
or a custom one needing the full assoc-type/supertrait/dispatch-penalty
calculation by checking the trait's name against a hardcoded 16-entry known
list (`Debug`, `Clone`, `Iterator`, …) — not by resolving what crate the
trait actually comes from. `TraitRegistryBuilder` does perform a two-pass,
whole-project scan so a trait's real shape (methods, associated types,
supertraits, impl count) is visible even when the trait is defined in a
different file than its impl — but that scan is still name-keyed pattern
matching over parsed syntax, not semantic resolution.

Hidden-dependency detection (`HiddenDepsCounter` in
`hidden_deps_counter.rs`) recognizes side-effecting calls by matching the
last one or two path segments against a known-call list
(`HIDDEN_DEP_TAILS`), gated by a `std`/`core` first-segment check for
longer paths — not by resolving what function is actually being invoked.

Because analysis never requires type information, `braintax` also never
requires the analyzed code to compile. `syn::parse_file` only needs valid
syntax, so `braintax` runs unchanged against a branch with unresolved
imports, missing dependencies, or an incomplete refactor mid-flight.

## Decision

`braintax` stays AST-only, built on `syn` alone. No dependency on `rustc`'s
internals (HIR/MIR/`rustc_middle::ty`) or on `rust-analyzer`'s semantic
layer is taken on, even though doing so would let both classification
mechanisms above resolve real types instead of matching names against a
list.

## Forcing constraints / Evidence

Both known-list mechanisms have live, self-disclosed blind spots that a
type-resolving analyzer would not have. Before suffix-based matching was
added (see `ADR-SeverityWeightedHiddenDeps.md`'s sibling fix), a
fully-qualified `std::time::Instant::now()` was silently missed by
`HiddenDepsCounter` because the list held only the unqualified form — a
concrete, reproduced bug, not a hypothetical one. Suffix matching narrowed
this considerably but did not remove the ceiling: a genuinely unknown
third-party trait or call still is not recognized, by design, since
recognizing it would require knowing what it actually resolves to.

## Rejected alternatives

**Depend on `rustc_driver`/the compiler internals directly.** Rejected:
unstable API, tied to a specific toolchain version, and would require the
analyzed crate to actually compile — defeating the "runs on any
syntactically valid source, compiling or not" property that makes
`braintax` usable mid-refactor and in editor-integration contexts.

**Depend on `rust-analyzer`'s IDE crates for semantic resolution.**
Rejected: a much larger dependency surface and a fundamentally different
architecture (incremental salsa-based query engine vs. a one-shot `syn`
walk) for a benefit — resolving trait/call identity correctly — that is
narrow relative to the cost, and partially addressable more cheaply
(suffix matching, list extension).

## Consequences

`braintax` has no compile requirement, no toolchain-version coupling, a
small dependency graph (`syn`, `quote`), and runs uniformly across any
target the source happens to be written for. In exchange, both
classification mechanisms have a permanent, accepted ceiling: a trait or
call not covered by their respective known-lists is misclassified, and no
future change within this architecture removes that ceiling entirely —
only narrows it, via list extension (done for hidden deps: `rand::random`,
`rand::thread_rng`, `process::abort`) or configurability (proposed, not
started, for formula weights generally).

## Enforcement

N/A — this is a foundational dependency choice, not a runtime-checkable
property. The check is `Cargo.toml` itself: no dependency on `rustc_*`
crates or `ra_ap_*`/`rust-analyzer` crates should ever appear.

## Related

- `ADR-SeverityWeightedHiddenDeps.md` — the suffix-matching fix that
  narrowed (but did not remove) this ADR's hidden-dep blind spot.
- `grip`'s own `ADR-AstOnlyNoTypeResolution.md` — the same decision,
  independently applicable to `grip`'s trait-boundary and hidden-dep
  detection, which share the identical `syn`-only shape.
