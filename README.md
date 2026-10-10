# cargo-braintax4rust

A cognitive tax estimator for Rust code — quantify the mental effort required to
understand a function.

`braintax` measures the cognitive load required to understand a piece of Rust code.
Not cyclomatic complexity. Not lines of code. Not nesting depth alone.

The **total price, in mental effort, that a reader pays to understand what a function
does, why it does it, and what it interacts with** — including everything the reader
must travel to outside the function itself to form a complete mental model.

---

## The formula, briefly

```
braintax = cyclomatic × cfg_factor × depth_factor × trait_factor
           + hidden_dep_weight + name_opacity + macro_density + generics
           + self_ref_cost + return_complexity
```

Complexity compounds: a function that's internally complex, buried deep,
gated behind cfg flags, *and* implementing an expensive trait is not
"complex + deep + gated + trait-heavy" — it's those four things at once,
so those four factors multiply. Everything else is an independent cost
layered on top, so it adds.

Every function also gets a normalized `braintax_normalized` (0–100, higher
= simpler), and every module/repo gets `total_braintax` — the sum across
every function in scope, plus its own `braintax_normalized`.

Full derivation of every term, every weight, and the severity table for
hidden dependencies: **[`docs/FORMULA.md`](docs/FORMULA.md)**.

---

## Documentation

| Doc | What's in it |
|---|---|
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | How a `braintax` invocation flows through the code, module by module. |
| [`docs/FORMULA.md`](docs/FORMULA.md) | Every scoring term, in full, kept in sync with `core/src/`. |
| [`docs/ADRs/`](docs/ADRs/) | Why the codebase is shaped the way it is. |
| [`docs/ROADMAP.md`](docs/ROADMAP.md) | What's shipped, what's next. |
| [`docs/OPEN_POINTS.md`](docs/OPEN_POINTS.md) | Known gaps, deliberately deferred. |
| [`CHANGELOG.md`](CHANGELOG.md) | Release history. |

---

## Installation

```sh
cargo install cargo-braintax4rust
```

## Development

```sh
just stage1
just stage2
```

Both must be green before a change is complete. Stage 1 is formatting, clippy
and tests — cargo built-ins only, so it works on a fresh checkout with none of
the tools below installed. Stage 2 is `cargo xtask stage2`, which runs, in
order: `cargo stern4rust` (house coding rules), `cargo dry4rust` (no duplication
in `core/src` beyond `dry4rust-baseline.json`), **braintax self-analysis**,
`cargo crap4rust` (complexity against coverage), `cargo twin4rust` (every source
file has a mirrored test file) and `cargo iceberg4rust` (file risk).

The self-analysis gate is this repository's own. It builds `cargo-braintax4rust`
from the working tree, points it at `core/`, and hands it a ceiling of **5.0**
on average brain tax — letting the tool render its own verdict rather than
re-implementing the comparison in the gate, so a change that costs this codebase
clarity is caught by the very measure the tool exists to report. `core/` scores
**4.0** today.

It measures `core/` specifically, not the whole tree. `fixture/` holds crates
written to score badly, because they are what braintax is *pointed at* — they
are the question, not the answer.

`xtask` is itself a workspace member and is gated like everything else. The
crate that runs the gates is not exempt from them.

Everything the two stages need, none of which ships with cargo:

| Tool | Install | Needed by |
|---|---|---|
| [`just`](https://github.com/casey/just) | `cargo install just` | both stages |
| [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov) | `cargo install cargo-llvm-cov` | stage 2 |
| `llvm-tools` rustup component | `rustup component add llvm-tools` | stage 2 |
| `cargo-stern4rust` | `cargo install cargo-stern4rust` | stage 2 |
| `cargo-dry4rust` | `cargo install cargo-dry4rust` | stage 2 |
| `cargo-crap4rust` | `cargo install cargo-crap4rust` | stage 2 |
| `cargo-twin4rust` | `cargo install cargo-twin4rust` | stage 2 |
| `cargo-iceberg4rust` | `cargo install cargo-iceberg4rust` | stage 2 |

`cargo-llvm-cov` and `llvm-tools` are what the CRAP gate needs; without them it
fails with a bare exit code that says nothing about a missing install.

`cargo-braintax4rust` itself is deliberately absent from that table — the gate
builds it from your checkout rather than taking an installed copy.

CI (`.github/workflows/ci.yml`) runs both stages on Ubuntu, Windows and macOS
for every pull request and every push to `main`.

## Usage

```sh
cargo braintax4rust [OPTIONS] [PATH]
```

**Arguments:**

| Argument | Description |
|---|---|
| `[PATH]` | Path to Rust crate or workspace root (default: `.`) |

**Options:**

| Option | Description |
|---|---|
| `--json` | Emit structured JSON output |
| `--threshold N` | Exit non-zero if any function's cyclomatic complexity exceeds N — checks the repo's `max_cyclomatic`, not the composite `braintax` score. Alias: `--max-complexity` |
| `--max-avg-braintax N` | Exit non-zero if the repo's `avg_braintax` exceeds N. Accepts a decimal. This is the composite score's own gate; combine it with `--threshold` and both must pass |
| `--top N` | Show the N most complex functions in the report (default: `10`) |
| `-h`, `--help` | Print help |
| `-V`, `--version` | Print version |

**Examples:**

```bash
cargo braintax4rust                              # current directory
cargo braintax4rust /path/to/crate               # specific path
cargo braintax4rust --json                       # structured output
cargo braintax4rust --threshold 10               # CI gate: fail if any function's CC > 10
cargo braintax4rust --max-avg-braintax 9.5       # CI gate: fail if avg braintax > 9.5
cargo braintax4rust --top 20                     # show the 20 most complex functions
cargo braintax4rust --json --threshold 10 --top 5
```

## Output

```
cargo-braintax4rust 0.11.0 -- core
══════════════════════════════════════════════

  Overall braintax:            4.0
  Maximum braintax:           15.2
  Total braintax:             360.3
  Normalized braintax:        73 / 100

Cyclomatic complexity:
  Total functions:             90
  Average complexity:         2.5
  Maximum complexity:         9
  Total complexity:           229

Per module:
  Module                          Funcs   Avg BT    Max
  ------------------------------  ------  --------  -----
  .                                90      4.0       15.2

Top 5 most complex functions:
  Function                                            Module          CC     BT    BT%
  --------------------------------------------------  ------------  -----  ------  -----
  core/src/default_scorer.rs                          .                3    15.2      0
  core/src/default_scorer.rs                          .                3    14.9      1
  core/src/collector.rs                               .                9    12.5     16
  core/src/fs_walk.rs                                 .                6    11.9     21
  core/src/collector.rs                                .                9    10.5     30
```

(`braintax`'s own `core/` source, analyzed by itself. `BT%` is that
function's own `braintax_normalized` — 0 at or above the CRAP-gate
ceiling, 100 at zero cost.)

### CI Gate

Use `--threshold N` to exit with code 1 if any function exceeds the maximum
cyclomatic complexity:

```bash
cargo braintax4rust --threshold 10
echo $?  # 0 if pass, 1 if fail
```

Use `--max-avg-braintax N` to gate on the composite score instead. It bounds
`avg_braintax`, the same figure `braintax_normalized` is derived from, but
unrounded and unclamped — so it keeps resolving above the normalization ceiling
of `15.0`, where the normalized score has already saturated to `0`:

```bash
cargo braintax4rust --max-avg-braintax 9.5
echo $?  # 0 if pass, 1 if fail
```

Both flags may be combined, in which case both bounds must hold. Either way the
report is still printed — the exit code is a verdict on the run, not a
replacement for its output.

---

## Limitations

- **AST-only, no type resolution.** Trait-factor and hidden-dependency
  classification are name/structure-based against a fixed known-list, not
  resolved against the trait's or call's actual origin. A third-party
  trait or call not on the list is invisible by design — see
  [`docs/ADRs/ADR-AstOnlyNoTypeResolution.md`](docs/ADRs/ADR-AstOnlyNoTypeResolution.md).
- **No cross-crate trait shape resolution.** `TraitRegistryBuilder` sees
  every trait definition and `impl` within the analyzed project, but not
  in its dependencies — a trait defined in an external crate falls back
  to the known-standard-trait or generic-custom-trait heuristic.
- **Formula constants are hand-picked, not empirically calibrated.** Every
  weight (severity values, penalty magnitudes, the normalization ceiling)
  reflects judgment, not measured correlation with actual comprehension
  time. See `OPEN_POINTS.md`'s "Configurable braintax formula weights".

---

## License

Licensed under MIT.
