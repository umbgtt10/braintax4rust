# Open Points

## Configurable braintax formula weights

Unlike grip's four cleanly-summing score weights, braintax's formula has
20+ independent hardcoded constants spread across `collector.rs` (cfg
base `2.0`, depth increment `0.15`, trait factor bases and penalties,
`self_ref_cost` `0.2`/`0.4`, `return_complexity` terms),
`hidden_dep_severity.rs`'s six-value severity table (`8.0` down to
`2.0`), the name-opacity/macro-density/generics counters, and
`braintax_normalizer.rs`'s ceiling (`15.0`). There's no single
"weights" struct to expose — making this configurable is a bigger job
than grip's equivalent: it needs collecting every constant behind one
`BraintaxWeights`/`FormulaConfig` struct, threaded through `Collector`,
`DefaultScorer`, `BraintaxNormalizer`, `HiddenDepSeverity`, and each
counter's constructor, before a CLI flag or config file means anything.

Same comparability trade-off as grip's version applies, amplified: with
this many independent knobs, two differently-configured runs could
diverge enough that "braintax score" stops meaning anything shared
across projects unless the config itself is captured in output.

Not started.

## Trait objects and `impl Trait` inside generic arguments

`return_complexity` looks through a reference and through `Box`, `Rc`
and `Arc`, so since 0.15.0 a trait object costs 1.0 where it is returned
behind its pointer. It does not look into any other type's generic
arguments: `Option<Box<dyn Iterator>>`, `Result<Box<dyn Error>, E>` and
`Option<impl Iterator>` each cost the 0.3 of a generic argument, never
the trait object or the opaque type inside. Recursing into every generic
argument would charge them, but would also move every nested generic --
`Vec<Vec<u32>>` from 0.3 to 0.6 -- so which it should be is the owner's
decision.

Not started.
