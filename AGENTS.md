# Working in RDF.rs

Rust 2024 workspace; declared MSRV 1.85. Work within this repository; do not inspect parent directories. Preserve unrelated user changes.

## Map

- `lib/rdf_rs`: facade, published as `rdf_rs`, commonly imported as `rdf`.
- `lib/rdf-model`: terms, triples/quads, patterns, interoperability; `lib/xsd`: datatypes and values.
- `lib/rdf-reader*`, `lib/rdf-writer*`: shared I/O APIs and format adapters.
- `lib/rdf-store`: transaction traits and heap store; `lib/rdf-store-*`: database adapters.
- `lib/rdf-format`, `lib/rdf-hash`, `lib/rdf-vocab`: format registry, hashes, vocabulary constants.
- `bin/rdf`: package `rdf-cli`, executable `rdf`.

## Implementation

- Read the target crate's `Cargo.toml` and `src/lib.rs` first. Confirm module declarations/re-exports: a file or feature name does not imply a working API. Many crates, examples, and methods are scaffolds.
- Shared versions/dependencies and local `[patch.crates-io]` entries live in root `Cargo.toml`. Follow workspace inheritance and existing module/re-export conventions.
- Do not add unsafe Rust. Preserve `no_std`, `alloc`/`std` gates and optional interop boundaries. Inherited dependency defaults currently enable `std` transitively; `--no-default-features` alone does not prove `no_std` support.
- Preserve RDF term kind, lexical form, datatype, language, direction, and graph through conversions. A quad's `None` context denotes the default graph; a pattern's `None` slot is a wildcard. Compare complete terms, not just `value_str()`.
- Keep streams lazy and propagate errors. Run blocking `StreamIter` consumption off Tokio workers (`spawn_blocking`). Read transaction semantics in `lib/rdf-store/src/{store,read_transaction,write_transaction}.rs`; preserve required `Send` bounds and document backend isolation/cancellation behavior accurately.
- Unsupported operations must return meaningful errors, not successful no-ops or fabricated empty results. Existing TODOs are not implementation examples.
- Treat persisted hashes, binary encodings, and database schemas as compatibility boundaries; test migrations or version changes.

## Checks

For Rust changes, substitute the Cargo package name:

```sh
cargo check -p <package> --all-targets
cargo test -p <package>
cargo clippy -p <package> --all-targets
cargo fmt --all -- --check
```

- Root CI runs `cargo build`, `cargo build --examples`, `cargo test`: only the 14 `default-members`. Select CLI, adapters, and `rdf-borsh` explicitly with `-p`; whole-workspace checks involve native/browser dependencies.
- For feature changes, also check the affected package with `--no-default-features`, with `--no-default-features --features alloc`, and with relevant interop features. Report existing failures precisely.
- IndexedDB tests: `wasm-pack test --headless --chrome` from `lib/rdf-store-idb`.
- Add focused regression tests for behavior fixes, including errors and RDF round trips. Avoid unrelated formatting or dependency churn.

## Documentation

- Document every public symbol added or changed in rustdoc; aim for complete public API coverage. Explain semantics, feature/runtime requirements, errors, and panics where relevant. Use runnable doctests or `no_run`; reserve `compile_fail` for intentional rejection examples.
- Prefer module/type rustdoc over README additions. Expand a README only for compelling user-facing value.
- Root and adapter READMEs are generated from `.config/codegen/` templates and crate `package.metadata.lvr` via `Rakefile` (Ruby 3.4+, `lvr`). Change these sources when correcting generated content.
- Check docs with `cargo doc -p <package> --no-deps`; `cargo test -p <package>` also runs doctests.
