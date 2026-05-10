# Changelog

All notable changes to `cargo-braintax4rust` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added

### Changed

### Fixed

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
