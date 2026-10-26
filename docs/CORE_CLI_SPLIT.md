# A technology-free core and a cli crate

Status: **in progress** -- one commit per slice, in the order below.

## Why

Mutation testing pays for every mutant with a build and a test run. A mutant in
code that walks a disk, spawns a process or prints is slow to kill, and a test
that needs a directory to analyse is slow to run: today two of core's tests
spawn `cargo run`, eleven analyse or write a directory on disk, and every
`App` test reads the report back from the JSON it was printed as. So the
counting, the scoring, the exclusions and the text report move into a crate
that touches no technology at all, and mutation testing is scoped to that crate
alone.

## The shape

| member | package | library | what it holds |
|---|---|---|---|
| `core/` | `braintax4rust-core` | `braintax` | `#![no_std]`, `#![forbid(unsafe_code)]`; the counting, the scoring, which files are read, the verdict and the text report. Depends on `libm` and `syn` only |
| `cli/` | `cargo-braintax4rust` | `braintax_cli` | the binary, `Args` (clap), the composition root `EntryPoint`, and every adapter: the file system, the JSON report, stdout |
| `xtask/` | `xtask` | `xtask` | the gates |
| `validation/` | `validation` | `validation` | the ordinal ranking over the fixtures, composed through `EntryPoint` |
| `fixture/*` | `fixture-*` | -- | analysis input, sources unchanged; each acceptance test composes through `EntryPoint` |

`cli` depends on `core`, never the reverse. Both published crates share one
workspace version. `test-utils` goes: once `App::run` hands the report back,
nothing needs `CaptureReporter` to capture it.

### Ports

Core declares a trait for everything technological it needs, implemented in
`cli/src/system/` and, for core's own tests, by in-memory fakes under
`core/tests/support/`:

| port (`core/src/traits/`) | what it answers | implementation |
|---|---|---|
| `SourceTree` | the `.rs` files under a root, and a file's text | cli `WalkdirSourceTree` |
| `Walk` | the files to analyse, with their text | core `SourceWalk`, over a `SourceTree`: it leaves out a build script and anything below a `target`, `.git`, `tests`, `examples` or `benches` directory, which is never read |
| `ReportRenderer` (was `Reporter`) | the report as text | core `TextReportRenderer`; cli `JsonReportRenderer`, with serde records of its own |
| `Scorer` | the overall and per-module figures | core `DefaultScorer` |

Core works on `SourcePath` -- a path held as text with `/` as its only
separator, read component by component the way `std::path` reads it, which cli
converts to at the edge -- and fails with typed errors (`SourceReadFailure`,
`RenderFailure`, `RunFailure`). `App::run` returns the report, its rendering and
the verdict; printing the rendering and turning the verdict into an exit code
is `EntryPoint`'s. `round` and the cfg factor's power of two come from `libm`,
bit for bit what `std` computes for every value measured.

The binary's behaviour does not change: stdout, stderr and exit codes are
compared byte for byte against 0.13.1 after every slice that touches them. One
exception is deliberate: a scanned `.rs` file whose path is not UTF-8 fails the
run naming it, where 0.13.1 scored it under a name it does not have. A file the
walk leaves out is still never read, whatever its name.

### The gate

A stage-2 gate, first in the run, keeps core technology-free: its manifest may
name only `libm` and `syn` in any section, its crate root must carry
`#![no_std]` and `#![forbid(unsafe_code)]`, and no file under `core/src` or
`core/tests` may reach `std::{env,fs,io,net,os,process}`, the printing macros,
the file-system methods or `extern crate std`.

### Mutation testing

`.cargo/mutants.toml` scopes cargo-mutants to `core/src`; each mutant runs
core's tests only. The fixtures, which are analysis input, are never mutated.

## Slices

| # | slice | status |
|---|---|---|
| S0 | this record | done |
| S1 | `DryGate`: one run over the workspace, with the trees it must not judge excluded by full path; the ignore file moves to the root it is read from | done |
| S2 | the split: `cli` takes `main.rs`, `Args` and `EntryPoint`; core is renamed; one workspace version; `[workspace.lints]` forbids unsafe code; `Config` becomes a record and the ignore file goes | |
| S3 | the `SourceTree` port, `SourcePath` and `SourceWalk`; the walk becomes cli's `WalkdirSourceTree` | |
| S4 | -- (braintax reads no cargo metadata: there is no workspace reader) | |
| S5 | the `ReportRenderer` port and `RunOutcome`: the text report in core, the JSON report in cli, printing and the exit code in `EntryPoint`; serde leaves core; `test-utils` goes | |
| S6 | `#![no_std]`: alloc imports, `libm`, typed errors; anyhow leaves the workspace | |
| S7 | the technology-free gate, first in stage 2 | |
| S8 | `.cargo/mutants.toml` | |
| S9 | docs, and this record moves to `docs/history/` | |

## Tests that move

Tests whose subject moves go with it, names kept:

| from | to |
|---|---|
| `core/tests/main_tests.rs` (2), `invocation/args_tests.rs` (4), `invocation/entry_point_tests.rs` (8) | the same files under `cli/tests/` |
| `invocation/config_tests.rs` (4) | `cli/tests/invocation/args_tests.rs`, through `Args::config` |
| `stdout_reporter_tests.rs`: the 2 text tests | `text_report_renderer_tests.rs` |
| `stdout_reporter_tests.rs`: the 2 JSON tests; `report_serializes_to_json`, `module_stats_serializes_to_json`, `overall_stats_serializes_to_json`, `function_complexity_serializes_to_json` | `cli/tests/system/json_report_renderer_tests.rs` |
| `fs_walk_tests.rs` (3, `fs_walk_*`) | `source_walk_tests.rs`, renamed `rust_files_*` |

The collector's and the registry's tests keep their names and assertions and
arrange `SourcePath`s; the `App` tests and the fixtures read the report `App::run`
returns instead of parsing the JSON a capturing reporter kept, the same values.
Where a test asserted `ExitCode::SUCCESS` or `ExitCode::FAILURE` from `App::run`,
it asserts the verdict `passes` instead; three JSON tests assert the pretty
document the renderer writes, so their literals gain the space after the colon.

Four tests are deleted, with the owner's approval:
`report_deserializes_from_json` and `function_complexity_deserializes_from_json`
pinned `Deserialize` on core's records, which serde's leaving core removes;
`new_agrees_with_default` and `new_starts_with_nothing_captured` go with
`test-utils`.
