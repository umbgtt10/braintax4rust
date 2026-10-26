# Braintax4Rust

## Meaning

`braintax` is a cargo subcommand that measures the cognitive load required to
understand a piece of Rust code — a composite score across complexity, cfg gating,
nesting depth, trait boundaries, hidden dependencies, naming, macros, and generics.

It is self-contained.

## Boundary Rule

This repository is **SELF-CONTAINED**.

The LLM **SHALL NOT cross its boundaries without asking**.

That means:
- do not inspect, edit, or rely on files outside `braintax/` unless the user explicitly asks
- do not pull assumptions from sibling repositories or crates
- do not propose cross-repository changes by default

## Quality Gates

### Mandatory after every change to `src/` or `tests/` of any crate in the workspace

Run gates:

`just stage1`
`just stage2`

If either gate is not green, the work is not complete.

Both run identically on Windows, Linux and macOS, and CI runs the same two
commands -- there is no second definition of the gates to drift out of step.

Stage 1 is formatting, clippy and tests -- cargo built-ins only, so it works on
a fresh checkout with none of the house tools installed. It lints test targets
as well as sources (`--all-targets`), which the PowerShell script did not.

Stage 2 is `cargo xtask stage2` -- a real crate under `xtask/`, gated like any
other code, rather than a script. Each gate is a `Gate` implementation
constructed against a `CommandRunner` trait, so the argument lists and the
failure messages are covered by `xtask`'s own integration tests. It runs eight
gates, in this order:

| gate | asks |
|---|---|
| Technology-free core | does `core` still touch no technology |
| `cargo stern4rust` | do the house coding rules hold |
| `cargo shear` | is every declared dependency used, in the section that uses it |
| `cargo dry4rust` | did this change add duplicated code |
| `cargo braintax4rust` | does this tool still pay its own bill |
| `cargo crap4rust` | is any function complex and untested |
| `cargo twin4rust` | does every source file have a mirrored test file |
| `cargo iceberg4rust` | is any file's private implementation risk too high |

The technology-free gate runs **first**. It is the xtask's own -- no tool to
install -- and asks three things of `core`: that its manifest names only
`libm` and `syn`, in any section (read from `cargo metadata --no-deps`; a
package missing from the metadata fails); that `core/src/lib.rs` carries
`#![no_std]` and `#![forbid(unsafe_code)]`, which hand `core/src` to the
compiler; and that no file under `core/src` or `core/tests` reaches
`std::{env,fs,io,net,os,process}`, a printing macro, a file-system method of
`Path` or `extern crate std`, read with syn so a string literal is data. The
`core/src` scan is a backstop to the compiler; `core/tests`, where std is
linked, depends on it. Calls inside a macro's arguments are not seen, and a
method of one's own named like a file-system one would be flagged. Its fix is
moving code into `cli`, which changes what every gate behind it measures, and
it takes a second.

stern4rust runs **second** because its corrections are renames, file moves and
directory splits: a layout it is about to reject is a layout the others would
have measured for nothing. Its findings are also the cheapest to act on.

All twenty-four of its rules apply to every member of the workspace, with
nothing skipped, nothing selected and nothing unconfigured. `docs/header.txt`
holds the three-line header every `.rs` file carries and `stern4rust.toml` names
it -- in the config rather than the gate script, so a hand-run of
`cargo stern4rust` checks exactly what the gate checks. Every member's manifest
carries `license = "MIT"`, which is what lets `spdx-matches-manifest` apply.

The stern gate judges every member, as a bare `cargo stern4rust` does --
`xtask`, the fixtures and `validation` included. The crate that runs the gates
is not exempt from them; it caught a misordered test in its own suite on the
first run. While the gate named only the published crate and `xtask`, the
fixtures broke `named-subject` six times with stage 2 green.

The one thing kept out of the scan is eight fixture source files that break a
rule by construction: a second type implementing the trait in the same file, a
public function nothing calls, a macro defined in `lib.rs`, a standard path
written out in full rather than imported. Each is excluded by name in
`stern4rust.toml`, beside the reason, rather than having a rule stood down; the
report counts every exclusion. Never "fix" one of them -- the shape of a
fixture's `src/` is the question braintax is asked, and its scores are pinned
twice over, by the fixture's own acceptance test and by the ordinal ranking in
`validation`. The acceptance tests under each fixture's `tests/` are not
analysis input (braintax reads nothing under `tests/`), so they follow the house
layout like any other suite.

cargo-shear runs **third**: its corrections edit manifests only -- a
dependency removed, or moved to the section that uses it -- so they change
nothing the gates behind it measure, and it takes a second. It runs `--locked`,
reading the lock file and never writing it, and `--deny-warnings`, so an
optional dependency no feature's code uses fails as an unused one does. A
dependency it cannot see used -- linked only for what it provides, declared
only to switch on a feature, used only by generated code -- goes in
`[package.metadata.cargo-shear] ignored`, and the gate refuses an entry without
its reason in the same table's `reasons`; there are none today. Pinned at
1.14.0, as CI installs it.

`dry4rust` runs **fourth** for the same reason stern4rust runs early: removing
a duplicate moves code between files and functions, which changes what every
gate behind it measures -- the braintax self-analysis included. It makes one run
over the workspace with the trees it must not judge excluded by full path --
`xtask`, the fixtures and `validation`, which are not published, and
`core/tests` and `cli/tests`, which repeat their arrangement by design -- so it
judges `core/src` and `cli/src` together and sees a copy from one into the
other. It checks with zero ceilings against `dry4rust-baseline.json`, the
duplication still there when the gate arrived. So it fails on what a change adds, not on what it inherited; a
duplicate removed is admitted, a copy added to a recorded group is not.

It counts only code units of 25 AST nodes or more. Below that sit the one-call
`Visit` overrides -- `visit_expr_while`, `visit_expr_try` and their siblings,
each bumping a counter and recursing -- and the one- and two-field
constructors; at dry4rust's default of 10 they form four groups whose sameness
is the trait's or the struct's rather than a copy.
At 25 the first run found one real copy: `MacroCounter`'s `visit_expr_macro`
and `visit_stmt_macro` each carried the same twelve lines of name lookup and
scoring, now shared as `MacroCounter::macro_cost`. What is left of the pair --
two four-line overrides `Visit` requires, one per node type -- is the baseline's
one entry.

Re-record the baseline only to drop groups that are gone, never to admit new
ones, and at the same floor -- a baseline matches only at the floor it was
recorded at: `cargo dry4rust --path "$PWD" --exclude "$PWD/xtask" --exclude "$PWD/fixture" --exclude "$PWD/validation" --exclude "$PWD/core/tests" --exclude "$PWD/cli/tests" --min-nodes 25 --baseline "$PWD/dry4rust-baseline.json" baseline`.

`cargo install just`
`cargo install cargo-llvm-cov`
`cargo install cargo-stern4rust`
`cargo install cargo-dry4rust`
`cargo install cargo-crap4rust`
`cargo install cargo-twin4rust`
`cargo install cargo-iceberg4rust`
`cargo install --locked cargo-shear --version 1.14.0`

cargo-braintax4rust is deliberately not in that list. The self-analysis gate
builds it from this checkout -- `cli`'s binary, pointed at `core/` and then at
`cli/`, each against the ceiling -- so the number it reports is the number for
the tree being changed rather than for whatever version happens to be
installed.

The measuring stage 2 gates are scoped to the two published crates,
`braintax4rust-core` and `cargo-braintax4rust` -- the dry gate judges their
`src/` directories -- which is what keeps the other members out of them; the
stern and dependency gates hold every member to the house rules and its
manifest, nothing more:

- the eighteen `fixture/` crates are analysis inputs, deliberately written to
  score badly. They are workspace members, so `cargo test` builds them and
  their own tests run, but no gate measures them.
- `validation/` holds the ordinal ranking test, which drives the analyser over
  every fixture in turn and asserts their scores stay in one fixed order. Its
  subject is the scoring model rather than any source file in `core/`, so it
  has no mirror there. Its own `src/` holds `FixtureAnalyzer`, the harness that
  runs one fixture end to end through `cli`'s `EntryPoint`, and the test file
  is named for that.

## Layout

A workspace with a virtual root that holds no package:

| member | what it is |
|---|---|
| `core/` | the analysis: package `braintax4rust-core`, library `braintax`. `#![no_std]`, `#![forbid(unsafe_code)]`, no technology -- see "Technology-free core" below. Published. |
| `cli/` | the command: package `cargo-braintax4rust`, library `braintax_cli`, binary `cargo-braintax4rust`. `Args`, the composition root `EntryPoint`, every adapter and the JSON report. Published. |
| `xtask/` | gate orchestration, run as `cargo xtask`. `publish = false`. |
| `validation/` | the ordinal ranking over the fixtures. `publish = false`. |
| `fixture/*` | eighteen crates of analysis input, each with its acceptance test. `publish = false`. |

`core` and `cli` share the workspace's `version`. `[workspace.lints.rust]`
forbids `unsafe_code` in `core`, `cli`, `xtask` and `validation`, every target;
`core` says so at its crate root as well. The fixtures do not opt in:
`base_hidden_deps` weighs an `unsafe` block by construction.

## Technology-free core

`core` touches no technology -- no file system, process, environment, argument
parsing, serialisation or stdout -- so mutation testing it needs nothing but its
own tests, which run in under a tenth of a second. `.cargo/mutants.toml` scopes
cargo-mutants to `core/src`, each mutant tested by core's tests only; the
fixtures, which are analysis input, are never mutated.

Whatever technology core needs is a port: a trait in `core/src/traits/`
(`SourceTree`, `ReportRenderer`); an in-memory fake for core's tests in
`core/tests/support/`; and the real adapter in `cli/src/system/`, composed in
`cli`'s `EntryPoint`. `Walk` and `Scorer` are ports too, with core's own
implementations, `SourceWalk` and `DefaultScorer`. Paths in core are
`SourcePath`s, text whose only separator is `/`, read component by component
the way `std::path` reads them; `cli`'s `PortablePath` converts at the edge,
and a scanned `.rs` file whose path is not UTF-8 fails the run naming it. The
text report is core's `TextReportRenderer`; the JSON one is cli's
`JsonReportRenderer`, with serde records of its own -- core has no serde. core's
failures are typed (`RunFailure` and the failures it wraps), so `EntryPoint`
prints `error: <message>` as it always has. `round` and the cfg factor's power
of two come from `libm`, bit for bit what the standard library computes.

## Publishing

`cli` publishes as `cargo-braintax4rust` so cargo resolves
`cargo braintax4rust`. The library `braintax` publishes as
`braintax4rust-core`, with a short README of its own.

A release publishes **both**, `braintax4rust-core` first, since `cli` names it
by version: `cargo publish --workspace` orders them and leaves the unpublished
members out. Their shared version moves once, in `[workspace.package]` and in
the root's `braintax` dependency together.

`cli`'s `Args::without_cargo_subcommand` strips the repeated subcommand name
that cargo inserts as `argv[1]`; running the binary directly does not repeat it,
so the strip is conditional, and it is tested.

Before publishing, `cargo publish --workspace --dry-run` must succeed.

## Orthogonality, trait surface and cognitive complexity

**When changing productive code, always maximize orthogonality and testable surface through traits, and minimize cognitive complexity.**

Specifically:
- prefer extracting behavior behind traits so individual pieces can be tested and swapped independently
- prefer small, focused methods with a single responsibility over large methods with many branches
- prefer named structs with methods over free functions operating on external state
- when `braintax` or a reviewer flags a function as too complex, reduce it by extracting internal structs with methods and adding integration coverage — not by extracting standalone helper functions
- never increase cognitive complexity to pass a test; find the root cause and fix it there
- when introducing a new protocol dependency seam, place the contract in `traits/`, place the protocol-facing state/data model parallel to the protocol, and place the concrete implementation in its own dedicated implementation area
- make constructors depend on traits, not directly on concrete implementations
- ALL dependencies are injected through the SINGLE constructor and stored in the struct
- apply the same split recursively to nested dependencies: trait first, state/data model second, concrete implementation third

## User coding standards

- one object per file; records where record-home puts them
- no unnecessary comments in code
- unit tests are not allowed. Only integration tests are
- consolidate scattered functions inside structs as appropriate
- no `&mut` input parameters; prefer return values
- only use `pub mod` in `mod.rs` and `lib.rs`
- split test files so there is one test file per source file, named `<source file name>_tests.rs`
- in `all_tests.rs`, reference test files one by one without `#[path = ...]`
- apply AAA (`Arrange`, `Act`, `Assert`) structure to tests with blank-line separation between the three sections
- use `// Arrange & Act` if there is no separate `Arrange`
- use `// Act & Assert` if there is no separate `Act`
- add the repository copyright and license header to every Rust source file
- tests should be named as follows `<method under test>_<test description>_<result>`
- all dependencies (both external and internal path deps) are declared in the workspace root `Cargo.toml` under `[workspace.dependencies]`
- member crates reference them via `dependency.workspace = true` — no version strings or path declarations in member Cargo.toml files
- do not use fully qualified paths; use `use` imports instead
