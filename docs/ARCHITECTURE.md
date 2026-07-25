# Architecture

How a `cargo braintax4rust` invocation actually flows through the code.
This is a map of what exists today, not a decision record — see
`docs/ADRs/` for the "why" behind the shapes described here, and
`docs/FORMULA.md` for how the scores themselves are computed.

---

## Pipeline

```
Args (clap)
  → Config
    → App
        walker:   Box<dyn Walk>      — finds .rs files
        scorer:   Box<dyn Scorer>    — aggregates + scores
        reporter: Box<dyn Reporter>  — renders output
      → App::run()
          1. collect_files():
             a. walker finds every .rs file under the target path
             b. Collector::build_trait_registry() does a whole-project
                pre-pass over all of them, building a name -> TraitInfo
                map (methods, associated types, supertraits, impl count)
                BEFORE any file is scored
             c. Collector::collect() is then called per file, using that
                registry, producing Vec<FunctionComplexity>
          2. compute_report(): scorer turns every function's braintax
             into OverallStats + per-module ModuleStats, assembles a
             BraintaxReport
          3. handle_output(): if --threshold is set, compare and exit;
             otherwise hand the BraintaxReport to the reporter
```

`App` (`app.rs`) is the only place that wires concrete types to trait
objects — everywhere else in the codebase depends on the `traits/`
interfaces, never on `FsWalk`/`DefaultScorer`/`StdoutReporter` directly.
See `docs/ADRs/ADR-DynDispatchAppOverGenerics.md` for why these are
`Box<dyn Trait>` fields rather than generic parameters.

The two-pass split (registry first, then scoring) exists specifically so
`trait_factor` is correct even when a trait is defined in a different
file than its `impl` — a single-pass design can only see a trait's shape
when both live in the same file, which is the uncommon case in a
realistically-organized crate.

## The three injected seams (`traits/`)

| Trait | Concrete impl | Responsibility |
|---|---|---|
| `Walk` | `FsWalk` | Recursively find `.rs` files under the target path, excluding `target/`, `tests/`, `examples/`, `benches/` by path component. |
| `Scorer` | `DefaultScorer` | Combine each function's `braintax` into `OverallStats`/`ModuleStats`; hold the `HiddenDepSeverity`/`BraintaxNormalizer` used per function. |
| `Reporter` | `StdoutReporter` | Render a `BraintaxReport` as human-readable text or JSON. |

Every fixture crate under `fixture/` exercises `App::with_deps()` with a
mix of real and fake implementations of these three traits — that's the
seam the whole test suite is built around. `test-utils`' `CaptureReporter`
is the one fake shared across all of them.

## Per-file analysis: `Collector`

`Collector::collect(source, path, root, traits)` parses one file's source
with `syn::parse_file` and walks the resulting AST
(`syn::visit::Visit`), producing one `FunctionComplexity` per
function/method found. `Collector::push_fn` is where every scoring
dimension comes together for a single function, delegating to a set of
smaller, single-purpose visitors as it goes:

| Helper | Question it answers |
|---|---|
| `ComplexityVisitor` | Cyclomatic complexity — how many decision points does this function body have? |
| `HiddenDepsCounter` (+ `HiddenDepSeverity`) | Which calls in this body are hidden dependencies, and how severe is each one? |
| `NameOpacityCounter` | How opaque are this function's parameter/local/loop/match-binding names? |
| `MacroCounter` (+ `DeriveAttrScorer`) | Which macro invocations, attributes, or `#[derive(...)]` entries are unfamiliar? |
| `GenericsCounter` | How many generic type/const parameters, and how many trait bounds on them? |

Trait-factor lookup, `self_ref_cost`, `return_complexity`, and cfg-gate
counting are computed directly in `Collector`/`collector.rs`, not
delegated — see `docs/FORMULA.md` for the exact formulas each of these
terms and helpers feed into.

## Building the trait registry: `TraitRegistryBuilder`

A separate struct (`trait_registry_builder.rs`), not part of the
per-file `Collector` walk. `Collector::build_trait_registry(files)`
delegates to it once per invocation, before any file is scored:

1. `scan_trait_definitions` — find every `trait Foo { ... }` across all
   files, record its method/associated-type/supertrait counts.
2. `scan_trait_impls` — find every `impl Foo for Bar { ... }` across all
   files, incrementing that trait's `impl_count`.
3. `record_impl` — the shared bookkeeping step both scans above call into.

The result (`HashMap<String, TraitInfo>`) is passed by reference into
every `Collector::collect` call for the rest of the run.

## Data model

| Type | Scope | Carries |
|---|---|---|
| `FunctionComplexity` | per function | `braintax`/`braintax_normalized` (this function's own score), `hidden_dep_weight`/`hidden_dep_labels`, `cyclomatic`, `trait_factor`, `depth` |
| `OverallStats` / `ModuleStats` | repo / module | `avg_braintax`, `max_braintax`, `total_braintax` (sum across every in-scope function), `braintax_normalized` |
| `BraintaxReport` | one CLI invocation | `overall`, `modules: Vec<ModuleStats>`, `functions: Vec<FunctionComplexity>` |

`BraintaxReport` is what `Reporter::render` turns into either the
human-readable summary or the `--json` output — it's the single shape
both output modes are projections of.

## CLI layer

`Args` (`args.rs`, `clap::Parser`) parses argv into flags; `Config`
(`config.rs`) is the plain-data form `App` actually consumes, built once
via `Config::from_args(args)`. `main.rs` and `lib.rs::run()`/`run_from_args()`
are thin entry points — all real logic lives in `App` and below.

## Related

- `docs/FORMULA.md` — how `braintax` and `braintax_normalized` are
  actually computed, term by term.
- `docs/ADRs/` — why `App` is shaped this way, why hidden deps are
  severity-weighted, and why classification is name/structure-based
  rather than type-resolved.
- `ROADMAP.md` — what's shipped and what's planned next.
