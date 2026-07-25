# ADR-DynDispatchAppOverGenerics

- **Status:** Accepted
- **Date:** 2026-07-25

## Context

`App`, the top-level orchestrator that wires `Walk`, `Scorer`, and
`Reporter` together and drives `run()`, originally carried a generic
parameter per dependency: `App<W: Walk, S: Scorer, R: Reporter>`. This is
the idiomatic zero-cost Rust shape — the compiler monomorphizes a distinct
`App` type per concrete `(W, S, R)` combination, so dispatch is static and
there is no vtable indirection.

In practice `App` is constructed via `App::new()` with the real
`FsWalk`/`DefaultScorer`/`StdoutReporter` in production, and via
`App::with_deps()` with fakes in every one of `braintax`'s 17 fixture
crates plus its own unit-level test suites. Every one of those call sites
that needed to *name* the type had to carry all three type parameters.

## Decision

`App` is a non-generic struct holding `Box<dyn Walk>`, `Box<dyn Scorer>`,
`Box<dyn Reporter>` fields. `with_deps()` takes those three boxes directly
as parameters, rather than generic or `impl Trait` parameters that get
boxed internally — construction is explicit at the call site.

## Forcing constraints / Evidence

N/A — this was a deliberate simplification, not a response to an external
constraint. The generic version compiled and worked correctly; it was
replaced because `braintax`'s actual usage never exercised more than two
concrete instantiations (production and test), and the CLI's own
per-invocation cost (a handful of file-system walks and AST parses) makes
vtable dispatch overhead immaterial.

## Rejected alternatives

**Keep the generics, accept the signature noise.** Rejected: no real
benefit at this call frequency for the cost of `App<W, S, R>` noise
wherever the type needed naming, most visibly across 17 near-identical
fixture-crate test helpers.

**`impl Trait` parameters on `with_deps()`, boxing internally.** Rejected
in favor of `with_deps(walker: Box<dyn Walk>, ...)` so the box is visible
and explicit at every construction site, matching the shape `grip` had
already landed on for the identical decision (see `grip`'s own
`ADR-DynDispatchAppOverGenerics.md`).

## Consequences

`App` is a single, nameable, ordinary type — usable in any signature
without generic parameters or turbofish. Two things fell out of dropping
the generics, both accepted:

`#[derive(Debug)]` was removed from `App`. `Box<dyn Trait>` is not `Debug`
unless the trait requires it as a supertrait, and none of `Walk`/`Scorer`/
`Reporter` do — `App` was never `{:?}`-printed anywhere, so this cost
nothing real.

`App::reporter()` (a test-only accessor that reached back into `App` to
read a `CaptureReporter`'s captured output after `run()`) became
impossible — there is no way to downcast `Box<dyn Reporter>` back to a
concrete type without adding `Any` to the trait bound. `CaptureReporter`
(the shared `test-utils` crate) moved from holding a plain `Mutex<String>`
to an `Arc<Mutex<String>>`, so every one of the 17 fixture crates plus the
core test suites could clone the shared handle *before* moving the
reporter into `App`, then read from that kept clone after `run()` — the
same pattern `grip` uses (`Rc<RefCell<String>>` there, since `grip`'s
fixtures define their own per-fixture `CaptureReporter` rather than
sharing one crate, and have no cross-thread requirement forcing `Arc`).

## Enforcement

N/A — structural; enforced only by the type signature of `App` itself
(there is no generic parameter left to reintroduce accidentally).

## Related

- `grip`'s own `ADR-DynDispatchAppOverGenerics.md` — the identical
  decision, made independently for `grip`'s own `App<W, S, R, C>` (one
  extra dependency, `CacheStore`, which `braintax` has no equivalent of).
