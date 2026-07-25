# The braintax formula

Full reference for how `braintax` computes its scores, at every level: per
function, per module, and per repo. `README.md` keeps a short summary and
points here for the complete picture — this is the document that stays in
sync with `core/src/`, not the other way around.

Every number here is read directly from the source that computes it
(`collector.rs`, `default_scorer.rs`, `hidden_dep_severity.rs`,
`braintax_normalizer.rs`, and the per-dimension counters), not transcribed
from memory — if this document and the source ever disagree, the source is
right and this file is stale. Two terms below (`self_ref_cost`,
`return_complexity`) are not in `README.md` at all today; this is the only
place they're currently documented.

---

## Shape: multiplicative core, additive penalties

```
braintax = cyclomatic × cfg_factor × depth_factor × trait_factor
           + hidden_dep_weight
           + name_opacity
           + macro_density
           + generics
           + self_ref_cost
           + return_complexity
```

The four compounding factors (cyclomatic, cfg, depth, trait) multiply,
because they compound: a function that is internally complex, buried
deep, gated behind cfg flags, *and* implementing an expensive trait is
not "complex + deep + gated + trait-heavy" — it's those four things
happening at once, and the reader pays for all of them simultaneously.
The remaining terms are independent costs layered on top, so they add.

Every term is computed in `Collector::push_fn` (`collector.rs`) per
function and combined by the free function `compute_braintax` in
`default_scorer.rs`.

---

## `cyclomatic` — base complexity

```
M = 1 + number of decision points
```

Decision points (`ComplexityVisitor`): `if`, `else if`, `while`, `for`,
`loop`, `match` arms, `&&`, `||`, `?`, `return`, `break`, `continue`.

## `cfg_factor`

```
cfg_factor = 2.0 ^ number_of_cfg_gates   (1.0 if zero gates)
```

Each `#[cfg(...)]`/`#[cfg_attr(...)]` attribute on the function counts as
one gate. One gate → `2.0`, two → `4.0`, three → `8.0`.

## `depth_factor`

```
depth_factor = 1.0 + (module_depth − 1) × 0.15
```

`module_depth` is the function's module nesting depth (crate root = 1). A
function at the crate surface: `depth_factor = 1.0`. Three modules deep:
`1.3`.

## `trait_factor`

Computed by `compute_trait_factor(name, info)`, where `info` comes from
`TraitRegistryBuilder`'s whole-project, two-pass scan (so the trait's real
shape is known even when it's defined in a different file than the
`impl`).

| Case | `trait_factor` |
|---|---|
| Known standard trait (`Debug`, `Clone`, `Copy`, `Default`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Hash`, `Display`, `Iterator`, `Into`, `From`, `Send`, `Sync`, `Drop`) | flat `0.80` |
| Inherent impl (no trait) | flat `0.95` |
| Custom trait | see below |

For a custom trait, `trait_factor` is built from four parts:

```
base            = 1.15 if (has associated types OR supertraits) else 0.90
                  + 0.01 × max(0, methods − 3)
dimension_penalty = 0.0 (neither) | 0.15 (one of assoc types/supertraits) | 0.20 (both)
dispatch_penalty  = min(0.27, min(0.18, 0.06 × max(0, impl_count − 1)) × amplifier)
                    amplifier = 1.5 if has associated types else 1.0

trait_factor = base + dimension_penalty + dispatch_penalty
```

A cheap, well-known trait boundary lets the reader stop at the boundary —
cost goes *down*. A custom trait with associated types, supertraits, and
several `impl` blocks increases cost, because the reader must track more
context to know which implementation applies at a given call site.

## `name_opacity`

Scored per parameter, local binding, `for`-loop variable, and `match`
binding (`NameOpacityCounter`):

| Identifier length | Penalty |
|---|---|
| 1 char (e.g. `x`) | +2 |
| 2–3 chars (e.g. `tmp`) | +1 |
| 4+ chars | 0 |

## `macro_density`

Each macro invocation, attribute, or `#[derive(...)]` entry outside a
known list costs **+3** (`MacroCounter`, `DeriveAttrScorer`):

- Known macros/attributes (cost 0): `println`, `eprintln`, `print`,
  `eprint`, `format`, `write`, `writeln`, `vec`, `assert`, `assert_eq`,
  `assert_ne`, `panic`, `unreachable`, `unimplemented`, `todo`,
  `include`, `include_str`, `include_bytes`, `concat`, `stringify`,
  `matches`, `cfg`, `file`, `line`, `column`, `compile_error`, `env`,
  `option_env`, plus the attributes `cfg_attr`, `allow`, `deny`, `warn`.
- Known derives (cost 0): `Debug`, `Clone`, `Copy`, `Default`, `Eq`,
  `PartialEq`, `Ord`, `PartialOrd`, `Hash`. Each *other* name inside a
  `#[derive(...)]` list costs +3 per name, independently of the macro
  list above.

Opaque or unfamiliar macros force the reader to go look up what they
expand to before trusting what the code does; the known lists are things
every Rust reader already carries in their head.

## `generics`

```
generics = Σ (2 + trait_bounds) per type param  +  3 per const generic
```

Each generic type parameter costs **+2**, plus **+1** per trait bound on
it (inline or in a `where` clause). Const generics cost **+3** each.
Lifetime parameters are free — they add no runtime behavior to reason
about.

## `hidden_dep_weight`

Not a flat per-occurrence penalty — a severity-weighted sum. See
`docs/ADRs/ADR-SeverityWeightedHiddenDeps.md` for why.

`HiddenDepsCounter` recognizes a fixed set of side-effecting patterns —
`unsafe` blocks, and calls whose last one or two path segments match a
known list, gated so a same-named third-party module can't
false-positive (`std`/`core`-prefixed or ≤2 segments only). Each match's
canonical label feeds `HiddenDepSeverity::severity(label)`:

| Severity | Labels |
|---|---|
| 8 | `unsafe` |
| 6 | `process::exit`, `process::abort`, `abort` |
| 5 | `fs::read`, `fs::write`, `File::open`, `File::create` |
| 4 | `Instant::now`, `SystemTime::now`, `random`, `rand::random`, `thread_rng`, `rand::thread_rng` |
| 3 | `env::var`, `env::args`, `thread::sleep` |
| 2 | `println`, `eprintln`, `print`, `eprint` |

`hidden_dep_weight` is the sum of every match's severity in that
function. `FunctionComplexity.hidden_deps: u32` is the plain count
(informational only — the formula uses `hidden_dep_weight`, not count).
`FunctionComplexity.hidden_dep_labels: Vec<String>` names every match, in
canonical short form, in the order found — so `std::time::Instant::now()`
and `Instant::now()` both report the label `"Instant::now"`.

## `self_ref_cost`

*(Not in `README.md` — documented here only.)*

| Receiver | Cost |
|---|---|
| none (free function or associated function) | 0.0 |
| `&self` | 0.2 |
| `&mut self` | 0.4 |

A method that mutates through `self` carries more state the reader must
track than one that only reads through it; a free function carries none
of this cost at all.

## `return_complexity`

*(Not in `README.md` — documented here only.)*

Computed by `type_complexity` on the function's return type:

| Return type shape | Cost |
|---|---|
| No return type (`()`, implicit) | 0.0 |
| `impl Trait` | 1.5 |
| `dyn Trait` (trait object) | 1.0 |
| Path type qualified as `<T as Trait>::...` (has a `qself`) | +1.0 |
| Path type starting with `Self` | +1.0 |
| Each generic argument on any path segment (`Vec<T>`, `Result<T, E>`, …) | +0.3 each |
| Plain concrete type (`i32`, `String`, a local struct by name) | 0.0 |

A function returning `impl Iterator<Item = T>` costs `1.5 + 0.3 = 1.8` —
the reader must both understand what the opaque type actually is *and*
track the generic parameter inside it.

---

## Normalization: `braintax_normalized`

```
braintax_normalized = round(100 × clamp(1 − braintax / 15.0, 0, 1))
```

Computed by `BraintaxNormalizer` (`braintax_normalizer.rs`). The ceiling
(`15.0`) matches the CRAP-gate threshold `scripts/run_stage_2.ps1` already
uses (`Invoke-Crap4RustGate -Threshold 15`) — a function at or above that
threshold normalizes to `0`; a function at `0` raw `braintax` normalizes
to `100`.

Every `FunctionComplexity` carries both its raw `braintax: f64` and
normalized `braintax_normalized: u32`. `OverallStats`/`ModuleStats` carry
`avg_braintax`, `max_braintax`, **`total_braintax`** (the sum, not
average, across every function in scope), and their own
`braintax_normalized`, derived from `avg_braintax` against the same
ceiling — not from averaging each function's own normalized value, which
would be a different (nonlinear) number.

**Decided: `TI = grip_absolute_total / total_braintax`**, not
`grip_score / braintax_normalized`. Raw sums are the ground truth — a
direct total of real per-function measurements, with no re-weighting or
clamping layered on top. `grip_score` and `braintax_normalized` are each
already a lossy 0–100 projection shaped by decisions specific to each
tool's own reporting needs: `braintax_normalized` above is derived from
`avg_braintax`, not `total_braintax`, discarding total codebase size
entirely; `grip_score` separately blends four independently-weighted
ratios. Dividing two independently-shaped normalizations would compound
their distortions into `TI`; dividing the two raw sums does not.

Not yet implemented — no released code computes `TI` today. The same
decision is recorded in `grip`'s own `FORMULA.md`.

---

## Related

- `docs/ADRs/ADR-AstOnlyNoTypeResolution.md` — why trait-factor and
  hidden-dep classification are name/structure-based rather than
  type-resolved, and the resulting blind spots.
- `docs/ADRs/ADR-SeverityWeightedHiddenDeps.md` — why hidden deps are
  weighted rather than flat, and where the severity table came from.
- `../OPEN_POINTS.md` — configurable formula weights.
- `fixture/` — every fixture crate is a worked example of one dimension
  in isolation; `fixture/base_hidden_deps` specifically demonstrates
  `hidden_dep_weight`/`hidden_dep_labels` end-to-end.
