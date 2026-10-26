# Architecture

How a `cargo braintax4rust` invocation actually flows through the code.
This is a map of what exists today, not a decision record — see
`docs/ADRs/` for the "why" behind the shapes described here, and
`docs/FORMULA.md` for how the scores themselves are computed.

---

## Pipeline

```
cli/src/main.rs
  → EntryPoint::run(argv)                     (cli)
    → Args (clap) → Args::config() → Config
    → EntryPoint::compose(config)
      → App::new(walker, scorer, renderer, config)     (core)
          walker:   Box<dyn Walk>           — SourceWalk over cli's WalkdirSourceTree
          scorer:   Box<dyn Scorer>         — DefaultScorer
          renderer: Box<dyn ReportRenderer> — TextReportRenderer (core) or,
                                              with --json, JsonReportRenderer (cli)
      → App::run() -> Result<RunOutcome, RunFailure>
          1. collect_files():
             a. the walker lists every .rs file under the target path and
                reads the ones it keeps
             b. Collector::build_trait_registry() does a whole-project
                pre-pass over all of them, building a name -> TraitInfo
                map (methods, associated types, supertraits, impl count)
                BEFORE any file is scored
             c. Collector::collect() is then called per file, using that
                registry, producing Vec<FunctionComplexity>
          2. compute_report(): scorer turns every function's braintax
             into OverallStats + per-module ModuleStats, assembles a
             BraintaxReport
          3. the renderer turns the report into text, and ThresholdGate
             gives the verdict -- the report is always rendered, whether
             or not a threshold is set
    → EntryPoint writes RunOutcome.output and a newline to stdout and
      turns RunOutcome.passes into the exit code
```

The workspace has two crates that matter here. `core/` (`braintax4rust-core`,
library `braintax`) is the analysis: `#![no_std]`, depending on `libm` and `syn`
only, touching no file system, process or stdout. `cli/`
(`cargo-braintax4rust`, library `braintax_cli`) is the command: the binary,
`Args`, `EntryPoint` and the adapters. `cli` depends on `core`, never the
reverse.

`EntryPoint::compose` (`cli/src/invocation/entry_point.rs`) is the only place
that wires concrete types to trait objects — everywhere else in the codebase,
`App` included, depends on the `traits/` interfaces, never on
`SourceWalk`/`DefaultScorer`/a renderer directly. `App::new` takes every seam
as a parameter and builds none of them, as stern4rust's
`injected-dependencies` rule asks; it builds only its concrete
`ThresholdGate`, from the bounds `Config` carries. See
`docs/ADRs/ADR-DynDispatchAppOverGenerics.md` for why these are
`Box<dyn Trait>` fields rather than generic parameters, and
`docs/ADRs/ADR-TechnologyFreeCore.md` for why the analysis is a crate of its
own.

The two-pass split (registry first, then scoring) exists specifically so
`trait_factor` is correct even when a trait is defined in a different
file than its `impl` — a single-pass design can only see a trait's shape
when both live in the same file, which is the uncommon case in a
realistically-organized crate.

## The seams (`core/src/traits/`)

| Trait | Implementation | Responsibility |
|---|---|---|
| `SourceTree` | cli `WalkdirSourceTree` | List the `.rs` files under a root, and read one. A root that does not exist lists nothing; an entry the walk cannot read is passed over. |
| `Walk` | core `SourceWalk`, over a `SourceTree` | The files to analyse, with their text: everything listed but a `build.rs` and whatever sits below a `target/`, `.git/`, `tests/`, `examples/` or `benches/` directory, by path component -- never read. |
| `Scorer` | core `DefaultScorer` | Combine each function's `braintax` into `OverallStats`/`ModuleStats`; with the `BraintaxNormalizer` for the overall and per-module figures. |
| `ReportRenderer` | core `TextReportRenderer`; cli `JsonReportRenderer` | Render a `BraintaxReport` as human-readable text, or as the `--json` document, whose serde records are cli's own. |

Paths in core are `SourcePath`s: text with `/` as the only separator, read
component by component the way `std::path` reads them, so a root such as
`core/.` still names its report `core`. cli's `PortablePath` makes one of a
platform path at the edge, lossily: a scanned file whose name is not UTF-8
cannot be read back under that name and fails the run naming it. Failures are
typed: `SourceReadFailure` ("failed to read <path>"), `RenderFailure`, and
`RunFailure`, which wraps them or says no Rust source was found; cli's
`EntryFailure` adds a failed write to stdout.

Core's tests use in-memory fakes from `core/tests/support/`
(`InMemorySourceTree`, `StubReportRenderer`) and touch no disk. Every fixture
crate under `fixture/` analyses its own directory through cli's
`EntryPoint::compose` and reads the report `App::run` returns -- the real
walk, scorer and renderer, end to end.

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

The result (`BTreeMap<String, TraitInfo>`) is passed by reference into
every `Collector::collect` call for the rest of the run.

## Data model

| Type | Scope | Carries |
|---|---|---|
| `FunctionComplexity` | per function | `braintax`/`braintax_normalized` (this function's own score), `hidden_dep_weight`/`hidden_dep_labels`, `cyclomatic`, `trait_factor`, `depth` |
| `OverallStats` / `ModuleStats` | repo / module | `avg_braintax`, `max_braintax`, `total_braintax` (sum across every in-scope function), `braintax_normalized` |
| `BraintaxReport` | one CLI invocation | `overall`, `modules: Vec<ModuleStats>`, `functions: Vec<FunctionComplexity>` |

`BraintaxReport` is what a `ReportRenderer` turns into either the
human-readable summary or the `--json` output — it's the single shape
both output modes are projections of. `App::run` returns it in a
`RunOutcome`, with its rendering and the verdict `passes`.

## CLI layer

`Args` (`cli/src/invocation/args.rs`, `clap::Parser`) parses argv into flags;
`Config` (`core/src/invocation/config.rs`) is the plain-data record `App`
actually consumes, built once by `Args::config()`. `main.rs` hands the
process's arguments to `EntryPoint::run`, which strips the name cargo repeats,
prints `error: <message>` and fails on an error, and calls
`EntryPoint::run_from_args`; that parses the arguments, runs the `App`
`EntryPoint::compose` builds, writes its output and maps its verdict to the
exit code. They are thin entry points — all real logic lives in `App` and
below.

`ThresholdGate` (`threshold_gate.rs`) owns the exit-code decision. It is
constructed from the two bounds `Config` carries — `threshold`
(`max_cyclomatic`) and `max_avg_braintax` — and answers one question,
`passes(&OverallStats) -> bool`. An unset bound always passes, so with neither
set the gate is vacuously true and the run exits `0`. Keeping it out of `App`
means the whole gating rule is reachable from a test without walking a
filesystem or scoring anything.

## Related

- `docs/FORMULA.md` — how `braintax` and `braintax_normalized` are
  actually computed, term by term.
- `docs/ADRs/` — why `App` is shaped this way, why hidden deps are
  severity-weighted, and why classification is name/structure-based
  rather than type-resolved.
- `docs/ROADMAP.md` — what's shipped and what's planned next.
