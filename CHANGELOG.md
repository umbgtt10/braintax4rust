# Changelog

All notable changes to `cargo-braintax4rust` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

## [0.11.1] - 2026-08-17

Documentation and CI only. No change to analysis, scoring or public API.

### Added
- `Invoke-Braintax4RustSelfGate` in `scripts/run_stage_2.ps1` — stage 2 now
  holds `braintax` to its own score, with a floor of 38 on
  `overall.braintax_normalized` that ratchets upward. Previously the
  self-analysis step ran `cargo run ... --json | Out-Null`, which discarded the
  report and so caught only an outright panic; the score could have collapsed
  without turning the gate red. The bound is a floor rather than a ceiling
  because `BraintaxNormalizer` inverts the cost — `(1 - braintax / 15) * 100`,
  so 100 is free to read and 0 is at the ceiling. The gate reads the JSON
  rather than passing `--threshold`, which compares `max_cyclomatic` rather
  than any braintax figure and, because `App::handle_output` returns before the
  reporter runs, exits non-zero having printed nothing to explain itself.
- `Invoke-Twin4RustGate` in `scripts/run_stage_2.ps1` — the mirrored-test rule
  is now enforced here, as it already was in `crap4rust`, `grip4rust`,
  `slotgate` and both `etheram` protocol repos. Only `cargo-braintax4rust` is
  gated: the fixture crates are analysis inputs whose purpose is to be small
  and odd, and `test-utils` is harness code. Already at zero gaps when wired.
  Requires `cargo-twin4rust` 0.2.0 or later.

### Fixed
- Links to `ROADMAP.md` and `OPEN_POINTS.md`, both of which moved into `docs/`.
  `README.md`'s documentation table pointed at the old repository-root paths,
  so both entries were dead links on crates.io and GitHub. Five prose
  references were also stale, in `README.md`, `docs/FORMULA.md`,
  `docs/ARCHITECTURE.md`, `docs/ADRs/README.md` and
  `docs/ADRs/ADR-SeverityWeightedHiddenDeps.md`.

## [0.11.0] - 2026-07-26

### Fixed
- `trait_info`'s supertrait count treated every bound in a trait's `: Bound1 +
  Bound2` clause as equally costly, including zero-cost auto-traits (`Send`,
  `Sync`, `Unpin`, `Sized`) that add no methods and no conceptual surface.
  Adding `: Send` to a trait purely to satisfy a clippy lint
  (`arc_with_non_send_sync`) inflated `trait_factor` — and therefore `braintax`
  — for every method of every implementor. `trait_info` now filters out known
  marker traits (and lifetime bounds) before counting supertraits, so only
  bounds that actually cost the reader something contribute to the dimension
  penalty. Found via empirical analysis of Faction's commit history.

## [0.10.0] - 2026-07-25

### Added
- `HiddenDepSeverity` struct — hidden dependencies are no longer counted
  as a flat penalty; each one is weighted by how much it actually
  compromises testability: `unsafe` +8, `process::exit`/`abort` +6,
  filesystem calls +5, time/randomness +4, env/thread calls +3,
  print-family +2. Matches the severity table `README.md` already
  documented but the formula never implemented.
- `FunctionComplexity` gains `hidden_dep_weight: f64` and
  `hidden_dep_labels: Vec<String>` — every hidden dependency's call
  site is now named in the output, not just counted.
- `docs/ADRs/` (index, `ADR-AstOnlyNoTypeResolution.md`,
  `ADR-DynDispatchAppOverGenerics.md`, `ADR-SeverityWeightedHiddenDeps.md`),
  `docs/ARCHITECTURE.md`, and `docs/FORMULA.md` — the authoritative
  formula and architecture reference, verified line-by-line against
  `core/src/`, including two terms (`self_ref_cost`, `return_complexity`)
  that were previously undocumented anywhere.
- `README.md`: a missing Installation section, and a full CLI
  Arguments/Options table (previously the Usage section was example
  invocations only, with no reference table and no mention of the
  `--max-complexity` alias for `--threshold`).

### Changed
- `compute_braintax`'s hidden-dep term is now the severity-weighted sum
  (`hidden_dep_weight`) instead of `hidden_deps * 4.0`. No fixture's
  expected `braintax` value changed — none of the 17 fixtures exercise
  the hidden-dep dimension — but any project with `unsafe` blocks or
  filesystem/process calls will now see a different (more
  differentiated) score than before.
- `README.md`: formula section trimmed to the headline equation plus a
  link to `docs/FORMULA.md`; the duplicate "Current phase"/"Roadmap"
  section (already diverged from `ROADMAP.md`) removed in favor of a
  Documentation nav table; the stale "Output" example (missing
  `Total braintax`/`Normalized braintax` and the `BT%` column) replaced
  with real captured output; new Limitations section added (previously
  had none, unlike grip's README).
- `ROADMAP.md` reconciled with shipped reality: every phase's status had
  been frozen at "Planned"/"In progress" since first written despite
  v0.2.0 through v0.8.0 having long since shipped; removed the unbuilt
  git-history/grip-integration phase; added the two phases (Generics
  v0.7.0, Trait refinement v0.8.0) that existed only as unlabeled
  Timeline rows with no write-up, plus this release.
- `docs/FORMULA.md`: the `grip / braintax` testability-index ratio,
  previously flagged as an open question, is now decided —
  `TI = grip_absolute_total / total_braintax` (raw sums, not the
  normalized `grip_score`/`braintax_normalized`). Still not implemented
  by any released code.
- `OPEN_POINTS.md`: fixed a stale reference to the flat
  `hidden_deps * 4.0` penalty (see the severity-weighting change above)
  that the "Configurable braintax formula weights" entry still
  described after it had already been replaced.

### Fixed
- Hidden-dependency detection matched exact full-path strings
  (`"Instant::now"`, `"env::var"` *and* `"std::env::var"` as separate,
  redundant entries) instead of suffix-matching the last two path
  segments the way grip's `HiddenDepFinder` does. A fully-qualified
  `std::time::Instant::now()` or a third-party-qualified
  `rand::random()`/`rand::thread_rng()` call was silently missed.
  `hidden_deps_counter.rs` now extracts the last two path segments and
  matches against a collapsed, short-form-only list, gated so a
  same-named third-party module (`mycrate::fs::read`) can't false-positive.
  Also adds `rand::random`, `rand::thread_rng`, and `process::abort` as
  newly-recognized hidden deps. New `fixture/base_hidden_deps` isolates
  the hidden-dep dimension end-to-end the way every other dimension
  already has its own fixture.

## [0.9.0] - 2026-07-25

### Added
- Per-function `braintax_normalized: u32` on `FunctionComplexity` — every
  function reports both its raw `braintax` score and a 0–100 normalized
  score.
- `total_braintax: f64` and `braintax_normalized: u32` on `OverallStats` and
  `ModuleStats` — summed/aggregate values, so a repo's braintax index can
  always be computed.
- `BraintaxNormalizer` struct — `100 × clamp(1 − braintax / 15.0, 0, 1)`,
  ceiling matches the CRAP gate's own threshold.
- "Total braintax" / "Normalized braintax" lines and a `BT%` column on the
  top-functions table in human-readable stdout output.
- `TraitRegistryBuilder` — two-pass trait registry built once across all
  files before any file is scored (see Fixed: cross-file trait factor).
- 8 new fixture crates plus `base_trait_multi`, completing full trait-factor
  scenario coverage (17 fixtures total), and an ordinal-ranking regression
  test across all of them.
- `app_new_with_empty_dir_returns_error` / `app_new_with_valid_dir_returns_success`
  — direct, in-process coverage of `App::new()`'s real dependency wiring,
  previously reachable only indirectly through CLI subprocess tests.

### Changed
- **Breaking:** `App<W: Walk, S: Scorer, R: Reporter>` is now a non-generic
  `App` holding `Box<dyn Walk>` / `Box<dyn Scorer>` / `Box<dyn Reporter>`
  fields. `with_deps()` takes those boxes directly. `App::reporter()` and
  `#[derive(Debug)]` removed.
- `CaptureReporter` (test-utils) holds `Arc<Mutex<String>>` instead of
  `Mutex<String>` so callers can keep a handle to the captured output after
  moving the reporter into `App`.
- Trait factor formula recalibrated for LLM-realism: inherent impl
  1.0→0.95; assoc/supertrait penalties now diminishing (+0.15 first, +0.05
  second) instead of flat +0.10 each; dispatch penalty changed from a step
  function to `0.06×(n−1)` capped at 0.18 (×1.5 amplified with associated
  types); method penalty 0.02→0.01 per method after the 3rd.
- `self_ref_cost` lowered: `&self` 0.5→0.2, `&mut self` 1.0→0.4 — the
  original weight dominated the score for otherwise-trivial methods.
- Added `return_complexity` to the formula (`Self` +1.0, `impl Trait` +1.5,
  `dyn Trait` +1.0, generics +0.3/arg).
- Internal `crate::` fully-qualified paths replaced with `use` imports
  throughout.
- Bumped version to 0.9.0.

### Fixed
- **Cross-file trait factor** (headline bug): `Collector` built a fresh,
  file-scoped trait registry per `collect()` call, so `compute_trait_factor`
  could only see a trait's real shape when the trait was defined in the
  same file as its impl. The ordinary cross-file pattern silently fell back
  to an empty `TraitInfo`, which happens to equal the same 0.90 as a
  genuinely-verified simple trait, so it never looked wrong. Fixed with a
  two-pass flow via `TraitRegistryBuilder`.
- `stdout_reporter.rs`: module table's Max column had no float precision,
  leaking values like `26.299999999999997` to the terminal.
- `stdout_reporter.rs`: top-functions table sorted by raw cyclomatic
  complexity instead of the composite `braintax` score.
- `fs_walk.rs`: same unanchored-substring exclusion bug as grip, same fix.

## [0.8.0] - 2026-05-10

### Added
- Phase 8: refined trait factor — associated types, supertraits, method methods count
- `TraitInfo` struct tracking methods, assoc types, supertraits
- `compute_trait_factor` with known-std discount (0.80) and custom trait model
- `base_trait_refined` fixture with `Extended: Base` (supertrait + assoc type)

### Changed
- Trait factor now accounts for: base (1.15), assoc types (+0.10), supertraits (+0.10), both (+0.10), extra methods (+0.02 each after 3rd)
- Known std traits (Debug, Clone, Iterator, etc.) get 0.80
- Bumped version to 0.8.0

## [0.7.0] - 2026-05-10

### Added
- Phase 7: generics scoring — type params (+2), trait bounds (+1), const generics (+3)
- `GenericsCounter` with where-clause support
- `BraintaxComponents` struct bundling all score dimensions
- `base_generics` fixture (CC=3, generics=9, braintax=12.0)
- 9 direct tests for generics scoring

### Changed
- Formula: `braintax = base × cfg × depth × trait + hidden + name + macros + generics`
- `compute_braintax_impl` refactored into `compute_braintax(&BraintaxComponents)`
- Bumped version to 0.7.0

## [0.6.0] - 2026-05-10

### Added
- Phase 5: user-defined macro density scoring (+3 per unknown macro)
- `MacroCounter` visitor: detects custom macros, excludes known std macros and derives
- `base_macros` fixture with custom `custom_add!` macro (CC=3, braintax=9.0)

### Changed
- Braintax formula: `braintax = base × cfg × depth × trait + hidden + name + macros`
- Known std macros/derives cost 0 — only user-defined macros add cognitive load
- Bumped version to 0.6.0

## [0.5.0] - 2026-05-10

### Added
- Phase 4: name opacity scoring — single-letter identifiers add cognitive cost
- `NameOpacityCounter` visitor: scores params, locals, for-vars, match bindings
- `base_opaque` fixture demonstrating poor naming (braintax 24.0 vs flat 18.0)

### Changed
- Braintax formula: `braintax = base × cfg × depth × trait + hidden + name_opacity`
- Existing fixtures use perfect naming (value, result, index) — no assertion changes
- Bumped version to 0.5.0

## [0.4.0] - 2026-05-10

### Added
- Phase 3: `depth_factor = 1.0 + (module_depth - 1) × 0.15`
- Phase 3: `trait_factor` — cheap (0.8), inherent (1.0), expensive (1.3)
- Depth tracked per function (module hierarchy depth)
- Trait definitions collected per file to assign method-count-based factors
- Functions inside `impl` blocks now receive the correct trait factor

### Changed
- Braintax formula now multiplies all four factors:
  `braintax = base × cfg × depth × trait + hidden`
- Fixture tests assert distinct values: flat 18.0, depth1 20.7, cfg 36.0, trait 14.4
- Bumped version to 0.4.0

## [0.3.0] - 2026-05-10

### Added
- Phase 2: `cfg_factor = 2.0 ^ gates` on `#[cfg]`-attributed functions
- Hidden dependency detection: `unsafe`, `println!`, `eprintln!`, `Instant::now`,
  `SystemTime::now`, `rand::random`, `thread_rng`, `File::open`, `std::fs::read/write`,
  `std::env::var`, `std::process::exit`, `abort`, `std::thread::sleep`
- `HiddenDepsCounter` visitor for detecting hidden deps in function bodies
- `cfg_body_gates` tracking in `ComplexityVisitor` for `#[cfg]` blocks inside bodies
- Composite `braintax` score: `cyclomatic × cfg_factor + hidden_deps_penalty`
- `FunctionComplexity.cfg_gates`, `.hidden_deps`, `.braintax` fields
- `ModuleStats` and `OverallStats` with `avg_braintax` / `max_braintax`
- Human output now shows braintax alongside cyclomatic complexity
- Separate `HiddenDepsCounter` tester with 7 tests
- Collector tests for `cfg_gates` counting and `unsafe`/`println!` detection

### Changed
- Bumped version to 0.3.0

## [0.2.0] - 2026-05-10

### Added
- Walk → Collector → Scorer → Reporter architecture pipeline with DI
- Cyclomatic complexity per function (McCabe formula `M = 1 + decision points`)
- CLI: `--json`, `--threshold N`, `--top N` flags
- Per-function tracking with `FunctionComplexity` struct
- Human-readable and JSON output with overall, per-module, and per-function breakdown
- CI gate via `--threshold N` (exit code 1 if max CC exceeds limit)
- Phase 2: `cfg_factor = 2.0 ^ gates`, hidden dependency detection
- 70 integration tests with fixture crates for four dimensions

### Changed
- Complete rewrite from v0.1.0 placeholder
- Crate renamed from `braintax` to `cargo-braintax4rust`
- Workspace structure: `core/`, `test-utils/`, `fixture/` crates

## [0.1.0] - 2026-05-08

### Added
- Initial placeholder release (`braintax` crate)
