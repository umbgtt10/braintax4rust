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
- Phase 2: `cfg_factor = 2.0 ^ gates`, hidden dependency detection
- 70 integration tests with fixture crates for four dimensions

### Changed
- Complete rewrite from v0.1.0 placeholder
- Crate renamed from `braintax` to `cargo-braintax4rust`
- Workspace structure: `core/`, `test-utils/`, `fixture/` crates

## [0.1.0] - 2026-05-08

### Added
- Initial placeholder release (`braintax` crate)
