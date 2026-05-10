# cargo-braintax4rust

A cognitive tax estimator for Rust code — quantify the mental effort required to
understand a function.

`braintax` measures the cognitive load required to understand a piece of Rust code.
Not cyclomatic complexity. Not lines of code. Not nesting depth alone.

The **total price, in mental effort, that a reader pays to understand what a function
does, why it does it, and what it interacts with** — including everything the reader
must travel to outside the function itself to form a complete mental model.

## Current phase (v0.2.0)

The current release computes **cyclomatic complexity** as the base dimension:

```
M = 1 + number of decision points
```

Where decision points include:
- `if` / `else if` expressions
- `while`, `for`, `loop` loops
- `match` arms (each arm beyond the first)
- `&&` and `||` boolean operators
- `?` (try) operators
- `return`, `break`, `continue` statements

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
cargo-braintax4rust 0.2.0 -- my-crate
══════════════════════════════════════════════

Overall cyclomatic complexity:
  Total functions:             42
  Average complexity:         3.2
  Maximum complexity:         15
  Total complexity:           134

Per module:
  Module                          Funcs   Avg     Max
  ------------------------------  ------  ------  -----
  lib                              15      2.3     7
  parser                           10      4.1     15
  utils                            8       1.5     3

Top 10 most complex functions:
  Function                                            Module          CC
  --------------------------------------------------  ------------  ----
  parser/src/parser.rs::parse_expression              parser          15
  lib/src/evaluator.rs::eval_deep                     lib             12
```

### CI Gate

Use `--threshold N` to exit with code 1 if any function exceeds the maximum
complexity:

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
| 2 | `cfg` | Feature gate multipliers, hidden dependency density |
| 3 | `depth` | Dependency travel distance, trait contract cost |
| 4 | Name opacity | Semantic distance between names and meaning |
| 5 | Macro density | Opaque macro invocations in productive code |
| 6 | Grip integration | Git history tracking, ratio diagnostics |

Complexity compounds. A function that is internally complex, buried deep,
gated behind cfg flags, and implementing an expensive trait is not "complex
+ deep + gated + trait-heavy." It is those four things at once — the cost
multiplies.

## License

Licensed under MIT.
