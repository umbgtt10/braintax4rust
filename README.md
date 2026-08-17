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
| `--top N` | Show the N most complex functions in the report (default: `10`) |
| `-h`, `--help` | Print help |
| `-V`, `--version` | Print version |

**Examples:**

```bash
cargo braintax4rust                              # current directory
cargo braintax4rust /path/to/crate               # specific path
cargo braintax4rust --json                       # structured output
cargo braintax4rust --threshold 10               # CI gate: fail if any function's CC > 10
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
