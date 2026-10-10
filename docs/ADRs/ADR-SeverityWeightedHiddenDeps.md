# ADR-SeverityWeightedHiddenDeps

- **Status:** Accepted
- **Date:** 2026-07-25

## Context

`HiddenDepsCounter` detects side-effecting calls inside a function body —
`unsafe` blocks, filesystem access, process control, time/randomness
queries, `println!`-family macros — and previously counted occurrences,
feeding a flat `hidden_deps × 4.0` penalty into `compute_braintax`. Every
hidden dependency cost the same regardless of what it actually was: a
function with one debug `println!` scored identically, on this term, to a
function with one `unsafe` block. `README.md` had already published a
severity table distinguishing these (`unsafe` +8 down to print-family +2)
before the formula caught up to it — the flat penalty was a genuine
implementation/documentation mismatch, not a considered simpler design.

Separately, `hidden_deps_counter.rs`'s call-recognition matched exact
full-path strings (`"Instant::now"`, `"env::var"` *and* `"std::env::var"`
listed as separate entries), so a fully-qualified `std::time::Instant::now()`
or a third-party-qualified `rand::random()` was silently missed — a
concrete bug hit while building this ADR's own fixture (`base_hidden_deps`
originally called `Instant::now()` fully-qualified and produced zero hidden
deps).

## Decision

`HiddenDepsCounter` now accumulates a severity-weighted `weight: f64` (via
`HiddenDepSeverity::severity(label)`) and records each match's
`labels: Vec<String>`, both driven by the label recognition described
below. `compute_braintax` uses `hidden_dep_weight` directly, replacing
`hidden_deps × 4.0`. `FunctionComplexity` exposes both
`hidden_dep_weight: f64` and `hidden_dep_labels: Vec<String>` in output, so
JSON consumers see which calls were flagged, not just how many.

Label recognition itself changed shape alongside the weighting: instead of
matching a path's full joined string against an exact-string list,
`hidden_tail` extracts the *last two* path segments and matches that tail
against a collapsed, short-form-only list (`HIDDEN_DEP_TAILS`), gated so a
3+ segment path only matches if its first segment is `std` or `core` — the
same approach `grip`'s `HiddenDepFinder` already used. This means a call's
label is always reported in canonical short form (`"Instant::now"`)
regardless of how it was qualified in source.

## Forcing constraints / Evidence

The fully-qualified `Instant::now()` miss is directly reproduced and fixed:
`hidden_deps_counter_tests.rs::fully_qualified_instant_now_counts_1` passes
today; it would have failed under the pre-fix exact-string list. The
severity table itself was already live in `README.md` before this decision
— implementing it, rather than leaving the mismatch, was not a judgment
call so much as closing a gap that already had a published answer.

## Rejected alternatives

**Special-case only `unsafe` to cost more, leave everything else flat.**
Smaller diff, touches fewer fixtures — but does not add labels, and only
fixes the single most obvious severity gap while leaving `println!` and
filesystem access indistinguishable.

**Port `grip`'s exact severity values (0.2–0.6) rather than deriving new
ones.** Rejected: `grip`'s scale is calibrated for its own 0–1
`contribution()` formula; `braintax`'s hidden-dep term is an additive
penalty on an unbounded score with a different existing anchor (the prior
flat `4.0`). `README.md`'s already-published table (`unsafe` +8 down to
print-family +2) was used as the source of truth instead, since it
predated this decision and already had its own internally consistent
scale.

**Copy grip's uppercase-identifier-not-a-safe-constructor catch-all
branch**, which would flag *any* unrecognized `Type::method()` call as a
hidden dependency. Rejected as out of scope: that is a materially larger
behavioral change (detecting unknown concrete-type construction generally,
not just a fixed catalog of known side-effecting calls) that changes what
"hidden dependency" means for `braintax`, not just how existing ones are
matched.

## Consequences

A function's hidden-dep score now reflects composition, not just count — a
repo that trades ten `println!`s for two `unsafe` blocks shows a
meaningfully worse score, which the flat penalty could not express. Every
severity value is still a hand-picked constant, same caveat as the rest of
`braintax`'s formula, and is explicitly named in the (postponed)
"Configurable braintax formula weights" open point rather than presented
as empirically derived. `fixture/base_hidden_deps` is the first fixture to
exercise this dimension end-to-end; the other 17 fixtures were and remain
unaffected, since none of them contain a hidden dependency of any kind.

## Enforcement

`hidden_dep_severity_tests.rs` and `hidden_deps_counter_tests.rs` pin the
severity value and detection behavior for every recognized category,
including the fully-qualified and third-party-qualified-must-not-match
cases. `fixture/base_hidden_deps/tests/lib_tests.rs` pins the
end-to-end `braintax` delta between a loaded and a clean function to the
severity sum.

## Related

- `ADR-AstOnlyNoTypeResolution.md` — this fix narrows that ADR's hidden-dep
  blind spot but does not remove it; an unrecognized third-party call is
  still invisible by design.
- `docs/OPEN_POINTS.md` — "Configurable braintax formula weights" covers making
  these severity constants (among many others) adjustable; deliberately
  postponed, not part of this decision.
