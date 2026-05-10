# Changelog

All notable changes to `cargo-braintax4rust` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added

### Changed

### Fixed

## [0.2.0] - 2026-05-10

### Added
- Walk → Collector → Scorer → Reporter architecture pipeline with DI
- Cyclomatic complexity per function (McCabe formula `M = 1 + decision points`)
- CLI: `--json`, `--threshold N`, `--top N` flags
- Per-function tracking with `FunctionComplexity` struct
- Human-readable and JSON output with overall, per-module, and per-function breakdown
- CI gate via `--threshold N` (exit code 1 if max CC exceeds limit)

### Changed
- Complete rewrite from v0.1.0 placeholder
- Crate renamed from `braintax` to `cargo-braintax4rust`

## [0.1.0] - 2026-05-08

### Added
- Initial placeholder release (`braintax` crate)

---

## v0.2.0 — Cognitive tax estimator

Initial architecture release with cyclomatic complexity as the base dimension.

### Architecture
- `Walk` trait + `FsWalk` with `target/`, `.git/`, `tests/`, `examples/`, `benches/` exclusion
- `Collector` parsing Rust source via `syn` with `#[test]`/`#[cfg(test)]` filtering
- `Scorer` trait + `DefaultScorer` for aggregation
- `Reporter` trait + `StdoutReporter` for human/JSON output
- `App<W, S, R>` generic over all three traits with `with_deps()` DI

### Cyclomatic complexity
- Decision points: `if`, `else if`, `while`, `for`, `loop`, `match` arms, `&&`, `||`, `?`, `return`, `break`, `continue`
- Functions inside `impl` blocks are collected (via `visit_impl_item`)

### Scoring
- Base formula: `braintax = cyclomatic × cfg_factor + hidden_deps_penalty`
- `cfg_factor = 2.0 ^ gates` (Phase 2)
- Hidden dependency detection: `unsafe`, `println!`, `Instant::now`, `File::open`, `rand::random`, `std::env::var`, `std::process::exit`, etc. (Phase 2)

### Data model
- `FunctionComplexity`: `name`, `file`, `module`, `cyclomatic`, `cfg_gates`, `hidden_deps`, `braintax`
- `ModuleStats`: per-module aggregation with CC and braintax
- `OverallStats`: project-wide aggregation with CC and braintax

### Project structure
- Workspace: `core/`, `test-utils/`, `fixture/base_*` crates
- `braintax-test-utils`: shared `CaptureReporter` for test binaries
- All dependencies in `[workspace.dependencies]`, referenced via `workspace = true`
- Fixture crates: `base_flat` (CC=18), `base_depth1` (module depth), `base_cfg` (cfg-gated), `base_trait` (trait impl)
- 70 integration tests, AAA-compliant, clippy-clean
- `scripts/run_stage_1.ps1` (fmt + clippy + test) and `run_stage_2.ps1` (self-analysis)
