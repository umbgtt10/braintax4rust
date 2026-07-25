# Open Points

## Configurable braintax formula weights

Unlike grip's four cleanly-summing score weights, braintax's formula has
15+ independent hardcoded constants spread across `collector.rs`
(hidden-dep penalty `4.0`, cfg base `2.0`, depth increment `0.15`, trait
factor bases and penalties, `self_ref_cost` `0.2`/`0.4`,
`return_complexity` terms), the name-opacity/macro-density/generics
counters, and `braintax_normalizer.rs`'s ceiling (`15.0`). There's no
single "weights" struct to expose — making this configurable is a
bigger job than grip's equivalent: it needs collecting every constant
behind one `BraintaxWeights`/`FormulaConfig` struct, threaded through
`Collector`, `DefaultScorer`, `BraintaxNormalizer`, and each counter's
constructor, before a CLI flag or config file means anything.

Same comparability trade-off as grip's version applies, amplified: with
this many independent knobs, two differently-configured runs could
diverge enough that "braintax score" stops meaning anything shared
across projects unless the config itself is captured in output.

Not started.
