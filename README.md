# cargo-braintax4rust

A cargo subcommand for computing cyclomatic complexity of Rust code.

## Usage

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

## Output

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

## Cyclomatic Complexity

Cyclomatic complexity (M) is computed as:

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

## CI Gate

Use `--threshold N` to exit with code 1 if any function exceeds the maximum
complexity:

```bash
cargo braintax4rust --threshold 10
echo $?  # 0 if pass, 1 if fail
```

## License

Licensed under MIT.
