# cargo-braintax4rust

A cognitive tax estimator for Rust code — quantify the mental effort required to
understand a function.

`braintax` measures the cognitive load required to understand a piece of Rust code.
Not cyclomatic complexity. Not lines of code. Not nesting depth alone.

The **total price, in mental effort, that a reader pays to understand what a function
does, why it does it, and what it interacts with** — including everything the reader
must travel to outside the function itself to form a complete mental model.

## Current phase (v0.4.0)

The current release computes a composite `braintax` score:

```
braintax = cyclomatic × cfg_factor × depth_factor × trait_factor + hidden_deps_penalty
```

### Cyclomatic complexity (base)

```
M = 1 + number of decision points
```

Decision points: `if`, `else if`, `while`, `for`, `loop`, `match` arms,
`&&`, `||`, `?`, `return`, `break`, `continue`.

### `cfg` factor

Each `#[cfg(...)]` gate on a function multiplies its score:

```
cfg_factor = 2.0 ^ number_of_cfg_gates
```

A function with one `#[cfg(feature = "...")]` gate has `cfg_factor = 2.0`.
Two gates → `4.0`, three → `8.0`.

### Depth factor

Each level of module nesting multiplies the score:

```
depth_factor = 1.0 + (module_depth - 1) × 0.15
```

A function at the crate surface: `depth_factor = 1.0`.
A function 3 modules deep: `depth_factor = 1.3`.

### Trait factor (non-monotonic)

Well-designed trait boundaries can *reduce* cognitive load:

| Trait type | Factor | Condition |
|---|---|---|
| Cheap trait | **0.8** | ≤3 methods, precise name |
| Inherent impl | **1.0** | No trait |
| Expensive trait | **1.3** | >3 methods or abstract name |

A cheap trait boundary (≤3 methods, single-word name) lets the reader
stop at the boundary — cognitive load goes *down*. An expensive trait
(abstract name, many methods) increases load because the reader must
track more mental context.

### Hidden dependencies

The tool detects side-effecting calls inside function bodies:

| Pattern | Penalty |
|---|---|
| `unsafe` block | +8 |
| `std::process::exit()`, `abort()` | +6 |
| `std::fs::read`, `File::open`, etc. | +5 |
| `Instant::now()`, `SystemTime::now()` | +4 |
| `rand::random()`, `thread_rng()` | +4 |
| `std::env::var()`, `env::args()` | +3 |
| `std::thread::sleep()` | +3 |
| `println!`, `eprintln!` | +2 |

### Usage

```bash
# Run on the current directory
cargo braintax4rust

# Run on a specific path
cargo braintax4rust /path/to/crate

# JSON output
cargo braintax4rust --json

# Set a maximum complexity threshold (exit code 1 if any function exceeds it)
cargo braintax4rust --threshold 10

# Show top N most complex functions
cargo braintax4rust --top 20

# Combined
cargo braintax4rust --json --threshold 10 --top 5
```

### Output

```
cargo-braintax4rust 0.3.0 -- my-crate

  Overall braintax:            13.2
  Maximum braintax:            36.0

Cyclomatic complexity:
  Total functions:             42
  Average complexity:         3.2
  Maximum complexity:         15
  Total complexity:           134

Per module:
  Module                          Funcs   Avg BT    Max
  ------------------------------  ------  --------  -----
  lib                              15      8.5      12
  parser                           10     21.0      36
  utils                            8       3.0       8

Top 10 most complex functions:
  Function                                            Module          CC     BT
  --------------------------------------------------  ------------  -----  ------
  parser/src/parser.rs::parse_expression              parser          15    36.0
  lib/src/evaluator.rs::eval_deep                     lib             12    12.0
```

### CI Gate

Use `--threshold N` to exit with code 1 if any function exceeds the maximum
cyclomatic complexity:

```bash
cargo braintax4rust --threshold 10
echo $?  # 0 if pass, 1 if fail
```

## Roadmap

The long-term model is multiplicative, not additive:

```
braintax = base × depth × cfg × trait + hidden + args + assoc + ...
```

| Phase | Dimension | Description |
|-------|-----------|-------------|
| 0.1 | Skeleton | Walk → Collector → Scorer → Reporter pipeline ✅ |
| 1 | `base` | Cyclomatic complexity, boolean chains, match arms, closures ✅ |
| 2 | `cfg` | Feature gate multipliers, hidden dependency density ✅ |
| 3 | `depth` | Dependency travel distance, trait contract cost ✅ |
| 4 | Name opacity | Semantic distance between names and meaning |
| 5 | Macro density | Opaque macro invocations in productive code |
| 6 | Grip integration | Git history tracking, ratio diagnostics |

Complexity compounds. A function that is internally complex, buried deep,
gated behind cfg flags, and implementing an expensive trait is not "complex
+ deep + gated + trait-heavy." It is those four things at once — the cost
multiplies.

## License

Licensed under MIT.
