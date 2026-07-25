# Open Points

## Hidden-dep path matching: align with grip's suffix-based approach

`hidden_deps_counter.rs::is_hidden_path()` matches exact full-path
strings (`"Instant::now"`, `"env::var"` *and* `"std::env::var"` listed
separately) rather than suffix-matching on the last N path segments the
way grip's `hidden_dep_finder.rs` does. A fully-qualified
`std::time::Instant::now()` or a third-party-qualified `rand::random()`
call is missed, since neither matches any listed exact string. Same
category of AST-only limitation as grip's foreign-trait allowlist gap —
worth aligning since both tools share an author and a purpose, not
because either approach is wrong on its own.

Not started.
