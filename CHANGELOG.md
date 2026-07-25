# Changelog

All notable changes to `cargo-braintax4rust` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

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
