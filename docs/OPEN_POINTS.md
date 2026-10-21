# Open Points

## A bare stern4rust run over the fixtures is not clean

Stage 2 runs stern4rust on `cargo-braintax4rust` and `xtask` alone; the fixture
crates stay out, each being a package shaped on purpose for braintax to be
pointed at. `stern4rust.toml` excludes by name the fixture files that break a
rule by construction, so that a bare `cargo stern4rust` over the whole workspace
reads clean too -- and since stern4rust 0.16.0 it does not. Its `named-subject`
rule reports `src/compute.rs` in six fixtures, `base_assoc_only`,
`base_inherent`, `base_many_methods`, `base_super_only`, `base_trait` and
`base_trait_refined`: each file is named after none of the types it implements.

Two ways to close it, and the choice is the owner's: exclude the six files by
name as the others are, leaving each fixture exactly as it is; or rename each
after the type it implements, a change to the fixture itself. The gate is
unaffected either way.

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
