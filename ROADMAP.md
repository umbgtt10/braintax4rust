# braintax — Roadmap

**Crate:** `braintax`  
**License:** MIT  
**Last updated:** 2026-07-25  
**Current status:** Phase 9 — ✅ Complete (v0.10.0 published)  

---

## Vision

`braintax` measures the cognitive load required to understand a piece of Rust code.

Not cyclomatic complexity. Not lines of code. Not nesting depth alone.

The **total price, in mental effort, that a reader pays to understand what a function
does, why it does it, and what it interacts with** — including everything the reader
must travel to outside the function itself to form a complete mental model.

**The core question `braintax` answers:**

*"What is the cognitive price of understanding this code?"*

---

## The fundamental insight

The relationship between structural features and cognitive load is **not monotonic**.

A trait boundary can reduce cognitive load — when it is cheap, precise, and carries
complete semantic meaning. The reader stops at the boundary. Travel distance: zero.

A trait boundary can increase cognitive load — when it is abstract, over-specified,
or introduces indirection without reducing surface. The reader must find implementors,
understand a large contract, track multiple supertrait bounds. Travel distance:
unbounded.

This means `braintax` cannot be reduced to a simple additive formula over structural
features. A multiplicative model (`base × depth × cfg × trait`) captures how
complexity compounds when multiple dimensions interact.

---

## What braintax measures

### Primary dimensions

**1. Internal complexity** (`base` — Phase 1)
What the reader must track inside the function body:
- Nesting depth — each level multiplies the context the reader must hold
- Boolean operator chains — long `&&` / `||` sequences require combinatorial reasoning
- Control flow jumps — `break`, `continue`, early `return`, `?` operator chains
- Match arm count — each arm is a case to hold simultaneously
- Function length — longer functions require more context window
- Closure depth — nested lambdas compound with the surrounding context

**2. `cfg` complexity** (multiplicative — Phase 2)
The combinatorial explosion of parallel universes:
- Each `#[cfg(feature = "...")]` gate on a function: ×2.0 multiplier
- Each `#[cfg(...)]` block inside a function body: proportional penalty
- Nested `cfg`: exponential

**3. Hidden dependency density** (additive — Phase 2)
Inputs the reader cannot see in the function signature:
- `Instant::now()`, `SystemTime::now()` — time is an invisible input
- `rand::random()`, `thread_rng()` — randomness is an invisible input
- `std::fs::*` — filesystem state is an invisible input
- `std::env::var()` — environment is an invisible input
- `println!`, `eprintln!` — output is an invisible side effect
- `std::process::exit()` — termination is an uncatchable side effect
- `unsafe` blocks — the reader cannot reason about safety without external context

**4. Dependency travel distance** (Phase 3)
How far the reader must travel outside the function to understand it.
The `depth` factor in `base × depth × cfg × trait`.

**5. Trait contract cost** (multiplicative, non-monotonic — Phase 3)
The trait factor in the product. A cheap boundary (≤3 methods, precise name)
has factor < 1.0 — it reduces cognitive load. An expensive boundary has
factor > 1.0 — it increases it. This is the non-monotonic dimension.

**6. Name opacity** (Phase 4)
The semantic distance between a name and its meaning. Single-letter identifiers
cost the most. Full semantic names are free.

**7. Macro density** (Phase 5)
Macros are opaque. Each macro invocation in productive code is a black box
the reader cannot see into.

### Multiplicative model

The core formula is a product, not a sum:

```
braintax = cyclomatic × cfg_factor × depth_factor × trait_factor
           + hidden_dep_weight + name_opacity + macro_density + generics
           + self_ref_cost + return_complexity
```

Complexity compounds. A function that is internally complex, buried deep,
gated behind cfg flags, and implementing an expensive trait is not "complex
+ deep + gated + trait-heavy." It's those four things at once. The cost
multiplies. Additive penalties (hidden dependencies, name opacity, macro
density, generics, self-reference cost, return-type complexity) are layered
on top.

This model is closer to how an LLM (or a human) experiences cognitive cost:
explaining a cfg-gated, deeply nested function requires visiting each variant's
logic separately, not summing difficulty scores.

Full derivation of every term and every weight: **[`docs/FORMULA.md`](docs/FORMULA.md)**.

---

## Phase 0.1 — Architecture skeleton

**Status:** ✅ Complete  
**Delivered:** as part of `braintax` v0.2.0 — not its own release; v0.1.1 never
shipped. v0.1.0 was an empty placeholder, and v0.2.0 was a complete rewrite
that folded this phase and Phase 1's internal-complexity baseline into one
release.

**Purpose:** Port the proven `grip` architecture (walk → parse → visit → score → report)
before writing a single line of scoring logic.

### Scope

- `src/main.rs` — calls `braintax::run()`
- `src/lib.rs` — module declarations only
- `src/args.rs` — clap Args (path, json, max_score, top)
- `src/config.rs` — Config from Args
- `src/walk.rs` — Walk trait + FsWalk impl (copy from grip)
- `src/collector.rs` — Collector with `syn::Visit`, extracts function names + metadata
- `src/scorer.rs` — Scorer trait + stub (returns zeros)
- `src/reporter.rs` — Reporter trait + StdoutReporter (stub)
- `src/app.rs` — App with `Box<dyn Trait>` injected deps
- `src/traits/` — Walk / Scorer / Reporter traits

### Gate

- Walks `cargo` source tree and prints function names + line counts
- JSON output works (all scores are 0)
- Stage 1 gate: fmt → clippy → tests, all green

---

## Phase 1 — Internal complexity baseline (the `base`)

**Status:** ✅ Complete  
**Delivered:** as part of `braintax` v0.2.0  

**The question Phase 1 answers:**

*"What is the raw internal cognitive mass of this function's body?"*

Phase 1 defines `base` — the structural complexity inside a function, before any
multipliers are applied. It does NOT produce a single composite score. It produces
six independent sub-dimension scores that will be folded into the multiplicative
formula in Phase 2.

### Design principle

Every score maps to a concrete, verifiable AST property. No normalization.
No calibration corpus. No target-codebase relativity. A nesting score of 18
means the same thing in a toy crate as in a 500K LOC system.

### Sub-dimensions (Phase 1)

**Nesting score:**
- Base: 0
- Each `if`, `else if`, `while`, `for`, `loop`, `match` block: +1 per nesting level
  (a nested `if` inside a `match` arm contributes +2, not +1)

**Boolean complexity:**
- Each `&&` or `||` in a condition: +1
- Each negated condition `!expr`: +1 (a full cognitive unit — the reader must invert)
- Double negation `!!expr`: +0 (the negations cancel)
- Negated compound `!(a && b)`: +1 for the negation, plus the inner complexity
- Nested boolean expressions (boolean inside boolean): +1 additional per level

**Control flow jumps:**
- Each `break` with a value: +1
- Each `continue`: +1
- Each early `return` (not the last statement): +1
- Each `?` operator: +0.5 (implicit early return, well-understood)

**Match complexity:**
- Each match arm beyond the first: +0.5
- A match arm with a guard (`if` condition): +1 additional
- A match arm with a nested pattern: +0.5 additional per nesting level

**Function length (continuous, no cliffs):**
```
length_score = max(0, lines - 10) × 0.15
```
This gives: 10 lines → 0, 20 lines → 1.5, 40 lines → 4.5, 80 lines → 10.5.
No step function. No incentive to shave below thresholds.

**Closure complexity:**
- Each closure: +1
- Each nested closure (closure inside closure): +2 additional per nesting level

### Output — no single score yet (superseded)

Phase 1, in isolation, did NOT produce a single braintax number — it produced raw
scores for each sub-dimension shown below. Phase 2 (next) folded these into the
first composite `braintax` score; every released version since has always shown
one. This section is kept as a record of the sub-dimension breakdown that still
feeds the composite score today.

```
braintax v0.2.0 — etheram-ibft
══════════════════════════════════════════════════════

Top 10 internal complexity:
  ibft/timer.rs::schedule_round_timeout
    nest: 18   bool: 4   jump: 6   match: 8   len: 10.5   closure: 0
  ibft/recovery.rs::import_recovered_blocks
    nest: 15   bool: 5   jump: 5   match: 10   len: 9.0   closure: 1

Highest nesting:  ibft/timer.rs::schedule_round_timeout (18)
Highest booleans: ibft/recovery.rs::process_message (9)
Highest closures: ibft/consensus.rs::handle_view_change (2)
Longest function: ibft/timer.rs::schedule_round_timeout (10.5)
```

JSON output includes the six sub-dimension fields per function.

### CLI interface

```
braintax [OPTIONS] [PATH]

Arguments:
  [PATH]    Path to Rust workspace or crate root [default: .]

Options:
  --json          Emit JSON output
  --max-nesting N     Exit 1 if any function nesting > N (CI gate)
  --max-boolean N     Exit 1 if any boolean complexity > N
  --max-length N      Exit 1 if any length score > N
  --top N             Show top N functions per dimension [default: 10]
  --module PATH       Restrict analysis to a specific module path
  -h, --help          Print help
  -V, --version       Print version
```

### Gate

- Builds with `--release`, runs in under 2 seconds
- `schedule_round_timeout` has highest nesting — known complex function
- No normalization — raw scores only
- CI gates `--max-nesting`, `--max-boolean`, `--max-length` all work
- Published on crates.io as `braintax` v0.2.0

---

## Phase 2 — `cfg` complexity + hidden dependencies (first composite score)

**Status:** ✅ Complete  
**Target:** 10–14 hours  
**Depends on:** Phase 1 complete  
**Deliverable:** `braintax` v0.3.0 on crates.io  

This phase introduces the `cfg` factor and produces the first composite braintax
score using the multiplicative model:

```
cfg_factor = 2.0 ^ number_of_cfg_gates_on_function

braintax = base × cfg_factor + cfg_body_score + hidden_dependency_score
```

### `cfg` factor

A function with one `#[cfg(feature = "x")]` has cfg_factor = 2.0.
A function with two gates has cfg_factor = 4.0.
A function with three gates has cfg_factor = 8.0.

Every cfg-gated code block inside the function body adds proportionally:

```
cfg_body_score = Σ (block_complexity × cfg_condition_complexity)

cfg_condition_complexity:
    simple feature flag:           0.5
    negated (not(feature)):        0.8
    combined (all(...)):           1.0
    combined (any(...)):           0.8
    nested combinations:           1.5 per nesting level
```

### Hidden dependencies

| Hidden dependency | Penalty |
|---|---|
| `Instant::now()`, `SystemTime::now()` | +4 |
| `rand::random()`, `thread_rng()` | +4 |
| `std::fs::read`, `File::open`, etc. | +5 |
| `std::env::var()`, `env::args()` | +3 |
| `println!`, `eprintln!` | +2 |
| `std::process::exit()`, `abort()` | +6 |
| `unsafe` block | +8 |
| `std::thread::sleep()` | +3 |

*(This flat table was the Phase 2 starting point. Phase 9 replaced it with a
severity-weighted lookup — see below — but the per-category values carried
forward unchanged.)*

### Output addition

```
ibft/timer.rs::schedule_round_timeout
  base: 60   cfg: ×1.0   hidden: Instant::now() +4, sleep +3
  braintax: 67

ibft/transport.rs::init_transport   [2 cfgs]
  base: 41   cfg: ×4.0   hidden: none
  braintax: 164
```

### Gate

- Phase 1 gate conditions still pass
- `init_transport` (or equivalent cfg-gated function) scores 2× to 4× its Phase 1 base
- `schedule_round_timeout` scores higher due to `Instant::now()` penalty
- `#[cfg(test)]` is always excluded from cfg analysis
- No false positives for `unsafe` detection
- Published on crates.io as `braintax` v0.3.0

---

## Phase 3 — Dependency travel distance + trait contract cost

**Status:** ✅ Complete  
**Target:** 8–12 hours  
**Depends on:** Phase 2 complete  
**Deliverable:** `braintax` v0.4.0 on crates.io  

The `depth` and `trait` factors in the multiplicative model.

### Dependency travel distance (`depth` factor)

```
depth_factor = 1.0 + (max_call_depth - 1) × 0.15
```

A function directly at the crate surface: depth_factor = 1.0.
A function 3 layers deep: depth_factor = 1.3.
A function 7 layers deep: depth_factor = 1.9.

### Trait contract cost (`trait` factor — non-monotonic)

```
trait_factor = 0.8  for cheap boundaries  (≤3 methods, precise name)
               1.0  for inherent impls
               1.3  for expensive boundaries (>3 methods OR abstract name)
```

The non-monotonic dimension: a well-designed trait boundary reduces cognitive load
(factor < 1.0) because the reader stops at the boundary. An expensive boundary
increases it (factor > 1.0).

*(These three flat tiers were the Phase 3 starting point. Phase 7 replaced them
with a shape-aware formula built from a trait's real method/associated-type/
supertrait count — see below.)*

### Gate

- Phase 2 gate conditions still pass
- At least one function's braintax decreases due to cheap trait boundaries
- At least one function's braintax increases due to expensive trait boundaries
- Published on crates.io as `braintax` v0.4.0

---

## Phase 4 — Name opacity

**Status:** ✅ Complete  
**Target:** 4–6 hours  
**Depends on:** Phase 3 complete  
**Deliverable:** `braintax` v0.5.0 on crates.io  

**Name opacity scoring:**

| Pattern | Score | Example |
|---|---|---|
| Single letter | +3 | `x`, `t`, `n` |
| Two letters | +2 | `tx`, `hs` |
| Opaque abbreviation | +2 | `mgr`, `hdl` |
| Generic verb without noun | +1 | `handle`, `process` |
| Qualified abbreviation | +0.5 | `peer_id`, `msg_count` |
| Full semantic name | +0 | `quorum_threshold` |

Added as a flat penalty on top of the multiplicative score.

*(This was the Phase 4 starting point; the shipped `NameOpacityCounter` scores by
identifier length instead — 1 char +2, 2–3 chars +1, 4+ chars +0. See
`docs/FORMULA.md#name_opacity` for the exact, current table.)*

### Gate

- Phase 3 gate conditions still pass
- Functions with single-letter parameters score higher than descriptive equivalents
- Published on crates.io as `braintax` v0.5.0

---

## Phase 5 — Macro density + composite score finalization

**Status:** ✅ Complete  
**Target:** 4–6 hours  
**Depends on:** Phase 4 complete  
**Deliverable:** `braintax` v0.6.0 on crates.io  

Macro invocations are opaque. Each one is a black box the reader cannot see into.

| Macro type | Score |
|---|---|
| Well-known stdlib (`vec!`, `format!`, `assert!`) | +0 |
| Project-local declarative macros | +2 |
| Procedural macros from external crates | +3 |

Added as a flat penalty.

*(Phases 6, 7, and 8 below added three more terms — generics, a refined trait
factor, and severity-weighted hidden deps — after this phase shipped. The
formula's final, current shape is in `docs/FORMULA.md`, not this section.)*

### Gate

- All previous gate conditions still pass
- Published on crates.io as `braintax` v0.6.0

---

## Phase 6 — Generics

**Status:** ✅ Complete  
**Delivered:** `braintax` v0.7.0

**What it adds:**

- `GenericsCounter` — each generic type parameter costs +2, plus +1 per trait
  bound (inline or in a `where` clause); each const generic costs +3;
  lifetime parameters are free
- `BraintaxComponents` struct bundling all score dimensions, replacing the
  ad hoc parameter list `compute_braintax_impl` had grown
- `base_generics` fixture (CC=3, generics=9, braintax=12.0)

---

## Phase 7 — Trait refinement

**Status:** ✅ Complete  
**Delivered:** `braintax` v0.8.0

**What it adds:**

- `TraitInfo` — tracks a trait's method count, associated types, and
  supertraits, so `trait_factor` reflects the trait's real shape instead of
  Phase 3's flat three-tier guess
- Known standard traits (`Debug`, `Clone`, `Iterator`, …) get a flat 0.80;
  custom traits are priced from a base plus dimension and method-count
  penalties
- `base_trait_refined` fixture (`Extended: Base` — supertrait + associated
  type together)

---

## Phase 8 — Absolute scores, cross-file trait factor, dyn dispatch

**Status:** ✅ Complete  
**Delivered:** `braintax` v0.9.0

**What it adds:**

- Cross-file `trait_factor` fix: `TraitRegistryBuilder` now scans every file
  for trait definitions and impls *before* any file is scored, so a trait's
  real shape is known even when it's defined in a different file than its
  `impl` — previously this silently fell back to an empty, coincidentally
  plausible-looking default
- `braintax_normalized`/`total_braintax` — every function and every
  module/repo now reports a normalized 0–100 score alongside the raw one
- `App` converted from `App<W, S, R>` generics to `Box<dyn Trait>` fields
  (see `docs/ADRs/ADR-DynDispatchAppOverGenerics.md`)
- `self_ref_cost`/`return_complexity` recalibration and 9 new fixture
  crates, bringing full trait-factor scenario coverage to 17 fixtures

See `CHANGELOG.md` [0.9.0] for the complete list.

---

## Phase 9 — Severity-weighted hidden deps, suffix-matched detection

**Status:** ✅ Complete  
**Delivered:** `braintax` v0.10.0

**What it adds:**

- `HiddenDepSeverity` — hidden dependencies are priced by actual severity
  (`unsafe` 8, `process::exit`/`abort` 6, filesystem 5, time/randomness 4,
  env/thread 3, print-family 2) instead of Phase 2's flat `× 4.0`
- Suffix-based path matching so fully-qualified calls
  (`std::time::Instant::now()`) or third-party-qualified calls
  (`rand::random()`, `rand::thread_rng()`) are no longer silently missed —
  previously matched exact full-path strings only
- `docs/ADRs/`, `docs/ARCHITECTURE.md`, `docs/FORMULA.md` — the
  authoritative formula and architecture reference

See `CHANGELOG.md` [0.10.0] and `docs/ADRs/ADR-SeverityWeightedHiddenDeps.md`
for the complete list and rationale.

---

## Timeline summary

| Phase | Deliverable | Key addition | Status |
|---|---|---|---|
| 0 | v0.1.0 | Placeholder | ✅ Complete |
| 0.1–1 | v0.2.0 | Architecture skeleton + cyclomatic complexity (base) | ✅ Complete |
| 2 | v0.3.0 | cfg factor + hidden dependencies | ✅ Complete |
| 3 | v0.4.0 | depth factor + trait factor (non-monotonic) | ✅ Complete |
| 4 | v0.5.0 | Name opacity | ✅ Complete |
| 5 | v0.6.0 | Macro density (user-defined macros) | ✅ Complete |
| 6 | v0.7.0 | Generics | ✅ Complete |
| 7 | v0.8.0 | Trait refinement — associated types, supertraits | ✅ Complete |
| 8 | v0.9.0 | Absolute scores, cross-file trait factor, dyn dispatch | ✅ Complete |
| 9 | v0.10.0 | Severity-weighted hidden deps, suffix-matched detection | ✅ Complete |

---

## Hard rules

- Every score maps to a concrete, verifiable AST property
- The formula is frozen per phase — no hand-tuning after publishing
- JSON output is never broken between minor versions
- CRAP score 0 before any phase is declared complete
- `#[cfg(test)]` blocks and `tests/` directories are always excluded
- Heuristics are documented as heuristics — `braintax` never claims to measure
  cognitive load exactly, only to approximate it through objective structural
  properties
