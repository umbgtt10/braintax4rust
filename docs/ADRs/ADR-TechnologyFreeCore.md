# ADR-TechnologyFreeCore

- **Status:** Accepted
- **Date:** 2026-10-26

## Context

The tool is meant to be mutation-tested, and mutation testing pays for every
mutant with a build and a test run. While one crate held everything, a mutant
anywhere rebuilt a crate that linked clap, serde, serde_json, walkdir and
anyhow, and its tests spawned `cargo run` twice, analysed or wrote eleven
directories on disk, and read every `App` result back from the JSON a capturing
reporter had kept. Run over the whole workspace, cargo-mutants also mutated the
eighteen fixture crates -- 1,138 of 1,699 mutants -- which are analysis input,
not code under test. The analysis -- the counting, the scoring, which files are
read and the verdict -- is what deserves mutating, and it was mixed with code
whose only job is to talk to the machine.

## Decision

The analysis lives in a technology-free crate, `braintax4rust-core`:
`#![no_std]` and `#![forbid(unsafe_code)]`, depending on `libm` and `syn` only,
declaring a trait for every piece of technology it needs, and returning its
report, its rendering and its verdict from `App::run`; a separate crate,
`cargo-braintax4rust`, implements those traits with the real technology, parses
the arguments, renders the JSON, prints, and decides the exit code.

## Forcing constraints / Evidence

`cargo mutants --list` names 456 mutants, all in the core, where the workspace
held 1,699 before the split. The core's 208 tests touch no disk and start no
process, and run in under a tenth of a second where the 199 before took 0.85 s;
its test binary went from 38.4 MB to 22.2 MB. Measured on the same machine, one
comparison in `ThresholdGate` changed as cargo-mutants changes it, medians of
three with other jobs holding its load average near six: building the crate
went from 0.60 s to 0.21 s, building and running its tests from 1.07 s to
0.37 s, and a clean build from 11.8 s to 3.9 s.

`round` and `powi` are not in `core` on the pinned compiler; `libm::round` and
`libm::exp2` agreed bit for bit with the standard library on every value
measured -- 2,001 powers of two and 42 million roundings -- so no score moved.
The binary's behaviour did not move either: stdout, stderr and exit codes were
compared byte for byte against 0.13.1 on twenty-seven invocations after every
step, with one deliberate exception: a scanned `.rs` file whose path is not
UTF-8 fails the run naming it.

## Rejected alternatives

- **One crate, with cargo-mutants' `exclude_globs` on the technology.** Scopes
  the mutants but not the build or the tests, and nothing would stop the next
  change reaching for `std::fs` inside the analysis.
- **serde behind an optional feature in core.** Keeps `Deserialize` on the
  report records, but core's manifest would still name serde and every
  workspace build would compile it in; the JSON document is cli's, with records
  of its own.
- **The exclusions in the disk adapter.** Which files are read is a decision
  worth mutating; the adapter only lists and reads, and `SourceWalk` decides.
- **iceberg4rust's `SourcePath` as it stands.** It compares `.` as written,
  which would name the report of a root such as `core/.` after `.`; braintax's
  reads `.` the way `std::path` does, measured against it on 3.9 million pairs
  of paths.

## Consequences

Two published crates, `braintax4rust-core` first, sharing one version. A new
technology need is a trait in `core/src/traits/`, an in-memory fake in
`core/tests/support/` and an adapter in `cli/src/system/`, composed in
`EntryPoint`. Paths in the core are `SourcePath`s with `/` as their only
separator, converted by `cli` at the edge. The core's errors are typed --
`RunFailure` and the failures it wraps -- and `EntryPoint` prints
`error: <message>` as it always did, so nothing in the workspace needs anyhow.
The core's report records no longer implement `Serialize` or `Deserialize`.
`test-utils` is gone: `App::run` hands back the report its tests used to
capture.

## Enforcement

The "Technology-free core" gate, first in stage 2: core's manifest names only
`libm` and `syn` in any section; `core/src/lib.rs` carries `#![no_std]` and
`#![forbid(unsafe_code)]`; no file under `core/src` or `core/tests` reaches
`std::{env,fs,io,net,os,process}`, a printing macro, a file-system method of
`Path` or `extern crate std`. `.cargo/mutants.toml` scopes cargo-mutants to
`core/src`, each mutant tested by core's tests only.

## Related

- [ADR-DynDispatchAppOverGenerics](ADR-DynDispatchAppOverGenerics.md)
- [`../ARCHITECTURE.md`](../ARCHITECTURE.md)
- [`../history/[IMPLEMENTED] CORE_CLI_SPLIT.md`](../history/%5BIMPLEMENTED%5D%20CORE_CLI_SPLIT.md)
