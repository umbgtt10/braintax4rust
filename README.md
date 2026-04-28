# braintax

A minimal cognitive tax estimator — quantify how mentally taxing code patterns are.

`braintax` provides a simple scoring function that combines nesting depth and branching complexity into a single heuristic value. It is designed as a lightweight companion to complexity analysis tools like `crap4rust`.

## Formula

```text
tax = nesting² + branches + 1
```

A score of **1** represents the baseline (no nesting, no branches). Higher scores indicate increasing mental taxation.

## Usage

```rust
use braintax::Braintax;

let score = Braintax::new()
    .with_nesting(3)
    .with_branches(5)
    .compute();

assert_eq!(score, 15); // 3² + 5 + 1 = 15
```

## License

Licensed under MIT or Apache-2.0 at your option.