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
failure messages are covered by `xtask`'s own integration tests. It runs six
gates, in this order:

| gate | asks |
|---|---|
| `cargo stern4rust` | do the house coding rules hold |
| `cargo dry4rust` | did this change add duplicated code |
| `cargo braintax4rust` | does this tool still pay its own bill |
| `cargo crap4rust` | is any function complex and untested |
| `cargo twin4rust` | does every source file have a mirrored test file |
| `cargo iceberg4rust` | is any file's private implementation risk too high |

stern4rust runs **first** because its corrections are renames, file moves and
directory splits: a layout it is about to reject is a layout the others would
have measured for nothing. Its findings are also the cheapest to act on.

All twenty-two of its rules apply to every member of the workspace, with
nothing skipped, nothing selected and nothing unconfigured. `docs/header.txt`
holds the three-line header every `.rs` file carries and `stern4rust.toml` names
it -- in the config rather than the gate script, so a hand-run of
`cargo stern4rust` checks exactly what the gate checks. Every member's manifest
carries `license = "MIT"`, which is what lets `spdx-matches-manifest` apply.

The stern gate is scoped to `cargo-braintax4rust` **and** `xtask`. The crate
that runs the gates is not exempt from them; it caught a misordered test in its
own suite on the first run. A bare `cargo stern4rust` judges every member --
the fixtures, `validation` and `test-utils` included -- and is green too.

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

`dry4rust` runs **second** for the same reason stern4rust is first: removing a
duplicate moves code between files and functions, which changes what every gate
behind it measures -- the braintax self-analysis included. It scans `core/src`
only -- tests repeat their arrangement by design -- and checks with zero
ceilings against `dry4rust-baseline.json`, the duplication still there when the
gate arrived. So it fails on what a change adds, not on what it inherited; a
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
recorded at: `cargo dry4rust --path core/src --min-nodes 25 --baseline "$PWD/dry4rust-baseline.json" baseline`.

`cargo install just`
`cargo install cargo-llvm-cov`
`cargo install cargo-stern4rust`
`cargo install cargo-dry4rust`
`cargo install cargo-crap4rust`
`cargo install cargo-twin4rust`
`cargo install cargo-iceberg4rust`

cargo-braintax4rust is deliberately not in that list. The self-analysis gate
builds it from this checkout, so the number it reports is the number for the
tree being changed rather than for whatever version happens to be installed.

Every stage 2 gate is scoped to `cargo-braintax4rust` -- the stern gate adds
`xtask`, and the dry gate is pointed at that crate's `core/src` -- which is what
keeps the other members out of them:

- the eighteen `fixture/` crates are analysis inputs, deliberately written to
  score badly. They are workspace members, so `cargo test` builds them and
  their own tests run, but no gate measures them.
- `validation/` holds the ordinal ranking test, which drives the analyser over
  every fixture in turn and asserts their scores stay in one fixed order. Its
  subject is the scoring model rather than any source file in `core/`, so it
  has no mirror there. Its own `src/` holds `FixtureAnalyzer`, the harness that
  runs one fixture end to end, and the test file is named for that.

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

- one struct per file
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
