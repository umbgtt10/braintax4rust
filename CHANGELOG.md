# Changelog

All notable changes to `cargo-braintax4rust` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

## [0.13.1] - 2026-10-10

How the repository is gated, not what the tool measures. No scoring rule
changed, no flag moved, and the output and exit codes are the same, so no
crate's brain tax moves. Patch rather than minor because no `pub` item of the
library changed signature: the one change under `core/src` adds a private
associated function to `MacroCounter`.

### Changed
- **Every stern4rust rule now applies to every workspace member.**
  `stern4rust.toml` stood down 79 rule-package pairs across the eighteen
  fixture crates and `validation`, and `spdx-matches-manifest` went
  unconfigured in nineteen members for want of a `license` field. All
  twenty-two rules now apply everywhere, with nothing skipped, selected or
  unconfigured; a bare `cargo stern4rust` scans 174 files and finds nothing.
  - Each fixture's acceptance test moved from `tests/analysis_tests.rs` to
    `tests/lib_tests.rs` (`compute_tests.rs` in `base_macros`), behind a
    `tests/all_tests.rs` and an `all_tests` test target, with its imports
    ordered and `serde_json::from_str` imported. braintax reads nothing under
    `tests/`, so no fixture score moved and no assertion changed.
  - `validation`'s fixture-running helper, a free function in its test file,
    is now `FixtureAnalyzer` in `validation/src`, and the ordinal ranking test
    is named for it: `fixture_analyzer_tests.rs`.
  - Eight fixture source files break a rule by construction -- several
    implementors of one trait in one file, a public function nothing calls, a
    macro defined in `lib.rs`, standard paths written out in full. They are
    the shapes braintax is pointed at, so each is excluded by name with its
    reason rather than corrected.
- Stage 2 gains a sixth gate, `cargo dry4rust`, second after the house rules:
  no duplicated code in `core/src` beyond `dry4rust-baseline.json`, counting
  units of 25 AST nodes or more. CI installs `cargo-dry4rust` with the other
  stage 2 tools.
- What it found is shared rather than copied. `MacroCounter`'s
  `visit_expr_macro` and `visit_stmt_macro` repeated the same twelve lines of
  macro-name lookup and scoring; that is now `MacroCounter::macro_cost`, and
  every macro costs what it did. The two overrides `Visit` requires are the
  baseline's one entry. `core/`'s own score moves from 4.1 to 4.0 because the
  shared code is measured once.
- `binary_prints_version` asserted the literal `0.13.0`, so the bump moved it
  to `0.13.1`.

## [0.13.0] - 2026-08-24

How the gates are run, not what the tool measures. No scoring rule changed and
no flag moved, so no crate's brain tax moves. Minor rather than patch because
the workspace gained a member and the test suite grew by 69.

### Added
- `xtask/`, a real crate replacing the stage 2 PowerShell script. Each of the
  five gates is a `Gate` implementation constructed against a `CommandRunner`
  trait, so the argument lists and failure messages are covered by 69
  integration tests rather than being unobservable shell.
- `.github/workflows/ci.yml`: both stages on Ubuntu, Windows and macOS, for
  every pull request and every push to `main`. CI runs `just stage1` /
  `just stage2` -- the same two commands a developer runs -- so there is no
  second definition of the gates to drift out of step.

### Changed
- Gates run through `just stage1` / `just stage2` on all three platforms.
- The stern gate now covers `xtask` as well as `cargo-braintax4rust`. The crate
  that runs the gates is not exempt from them, and it earned its place
  immediately: the first stage 2 run failed on a misordered test in `xtask`'s
  own suite. The fixture crates stay out of the gate, as before.
- The braintax self-gate hands its ceiling to the tool rather than judging the
  average itself, so there is one implementation of that comparison and it is
  the one that ships.
- **The self-gate now measures `core/` rather than the whole tree, at a ceiling
  of 5.0 rather than 9.03.** It had no target argument, so it defaulted to the
  working directory and swept in `fixture/` -- 79 of the 178 functions it
  scored, at 26 to 52 brain tax each, from crates written to score badly
  because they are what braintax is pointed at. They outvoted the tool's own
  code: 8.0 overall against `core/`'s 4.1.

  That only became visible when the gate started running somewhere other than
  Windows. `fixture/base_assoc_only` scores 41.2 on macOS and 38.8 on Linux --
  same 178 functions, same cyclomatic complexity to the decimal, different
  brain tax -- which pushed the whole-tree average across 9.03 on macOS alone.
  Every `core/src` module is identical on both platforms, so scoping the gate
  to `core/` removes the disagreement rather than hiding it.

  The platform-dependence in that fixture is a real finding about the tool and
  is not addressed here.
- Stage 1 now lints test targets too (`cargo clippy --workspace --all-targets`),
  which the PowerShell script never did.
- CI checks formatting instead of applying it (`cargo fmt --check` when `CI` is
  set), so drift fails the build rather than being silently rewritten where
  nobody is there to review it. A local `just stage1` still formats in place.
- `binary_prints_version` asserted the literal `0.12.0`, so the bump moved it to
  `0.13.0`. Left as a literal rather than switched to `env!("CARGO_PKG_VERSION")`:
  the binary derives its version from that same constant, so the comparison
  would pass whatever either side reported.

### Removed
- `scripts/run_stage_1.ps1` and `scripts/run_stage_2.ps1`. A Windows-only gate
  is not a gate contributors on Linux or macOS can run.

## [0.12.0] - 2026-08-17

The composite score gets a CI gate of its own, and setting a threshold no
longer swallows the report.

### Added
- `--max-avg-braintax N` — exit non-zero when the repo's `avg_braintax`
  exceeds `N`. Until now the only gate was `--threshold`, which compares
  `max_cyclomatic`; the composite score this tool exists to compute could not
  fail a build at all. It bounds `avg_braintax` rather than
  `braintax_normalized` because the two are the same measurement —
  `DefaultScorer` computes `braintax_normalized` as
  `normalize(avg_braintax)` — but the raw average is neither rounded to an
  integer nor clamped, so it keeps resolving above the normalization ceiling of
  `15.0`, where the normalized score has already saturated to `0`. Accepts a
  decimal. Combines with `--threshold`: both bounds must hold.
- `ThresholdGate` — the exit-code decision, extracted from
  `App::handle_output` into a struct that takes the two bounds and answers
  `passes(&OverallStats) -> bool`. An unset bound always passes. Reachable from
  a test without walking a filesystem or scoring anything, which
  `App::handle_output` was not.

### Fixed
- Setting a threshold no longer suppresses the report. `App::handle_output`
  returned the exit code before reaching `self.reporter.write(report)`, so
  `--threshold 10` printed nothing at all — on failure it exited `1` with no
  indication of which function was too complex, or by how much. The report is
  now always written and the exit code is decided afterwards. This is why
  `scripts/run_stage_2.ps1` previously parsed `--json` by hand rather than
  using the tool's own gate; it now calls `--max-avg-braintax` directly.

### Changed
- `Config` carries `max_avg_braintax: Option<f64>` alongside `threshold`.

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
