# Architecture Decision Records

Each ADR documents one load-bearing decision behind `cargo-braintax4rust` —
succinct, self-contained, citable on its own. Unlike the larger `etheram`
ecosystem repos, these are not priority-tiered; `braintax` is a
single-crate CLI tool with a small enough decision surface that a flat
list is sufficient.

Further deferred/unstarted design decisions are tracked in
`../../OPEN_POINTS.md`, not here — an ADR records a decision already made,
not one still being weighed.

## Index

| ADR | Decision |
|---|---|
| [ADR-AstOnlyNoTypeResolution](ADR-AstOnlyNoTypeResolution.md) | `braintax` analyzes via `syn` AST parsing only, never type resolution — trait-factor and hidden-dep classification is name-based, with known, accepted blind spots for anything off the hardcoded known-lists. |
| [ADR-DynDispatchAppOverGenerics](ADR-DynDispatchAppOverGenerics.md) | `App` holds `Box<dyn Trait>` fields rather than generic type parameters — one nameable type instead of `App<W, S, R>` noise at every one of 17 fixture-crate call sites. |
| [ADR-SeverityWeightedHiddenDeps](ADR-SeverityWeightedHiddenDeps.md) | Hidden dependencies cost a severity-weighted sum (`unsafe` +8 down to print-family +2), not a flat per-occurrence penalty — matches what `README.md` already documented. Bundled with a suffix-based path-matching fix for the same detection code. |

## Template

```markdown
# ADR-<Name>

- **Status:** Accepted | Proposed | Superseded by <ADR>
- **Date:** YYYY-MM-DD

## Context
The forces and tension this resolves.

## Decision
The choice, in one quotable sentence.

## Forcing constraints / Evidence
Why this was forced, not freely chosen — the real evidence. `N/A` if none.

## Rejected alternatives
What we did not do, and why.

## Consequences
What it commits us to; what it costs; obligations pushed onto consumers.

## Enforcement
The specific test, gate, or structural mechanism that keeps it true.
`N/A` if purely structural.

## Related
Links to other ADRs (this repo or `grip`) and architecture docs.
```

Fields that do not apply are marked `N/A` rather than padded. Each ADR is a
snapshot of the decision as it stands today, not a changelog — state the
current shape as fact, don't narrate what an earlier version of this
document used to say.
