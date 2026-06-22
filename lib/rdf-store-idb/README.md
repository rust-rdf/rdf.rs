# RDF.rs: IndexedDB Store

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/rdf-store-idb)](https://crates.io/crates/rdf-store-idb)
[![Documentation](https://docs.rs/rdf-store-idb/badge.svg)](https://docs.rs/rdf-store-idb)

An [IndexedDB] storage adapter for **[RDF.rs]** knowledge graphs.

> [!TIP]
> 🚧 _We are building in public. This is presently under heavy construction._

<sub>

[[Features](#-features)] |
[[Prerequisites](#%EF%B8%8F-prerequisites)] |
[[Installation](#%EF%B8%8F-installation)] |
[[Examples](#-examples)] |
[[Reference](#-reference)] |
[[Development](#%E2%80%8D-development)]

</sub>

## ✨ Features

- Implements a high-performance RDF store backed by [IndexedDB].
- Built on async Rust using lazily-evaluated streams throughout.
- Plays nice with others: interoperates with Oxigraph, Rudof, and Sophia.
- 100% pure and safe Rust with minimal dependencies and no bloat.
- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- Cuts red tape: 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition)

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add rdf-store-idb
```

### Installation in `Cargo.toml`

Enable all default features:

```toml
[dependencies]
rdf-store-idb = { version = "0.4" }
```

Enable only specific features:

```toml
[dependencies]
rdf-store-idb = { version = "0.4", default-features = false, features = ["tracing"] }
```

## 👉 Examples

### Importing the Library

```rust
use rdf_store_idb::{IdbStore, IdbTransaction};
```

### Querying the Store with SPARQL

To execute SPARQL queries on the store, use the [sparql-store] crate to wrap
the underlying [`IdbStore`] quad store into a [`SparqlStore`]:

```rust,compile_fail
use sparql_store::SparqlStore;

let mut store: SparqlStore<IdbStore> = ... // TODO

let tx = store.read().await?;
```

## 📚 Reference

[docs.rs/rdf-store-idb](https://docs.rs/rdf-store-idb)

### Storage Schema

The current implementation is based on a triple-centric schema where triples
are uniquely identified and deduplicated by their subject-predicate-object
(SPO) hash. Graphs, in turn, are represented as sets of triple IDs.

## 👨‍💻 Development

```bash
git clone https://github.com/rust-rdf/rdf.rs.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-idb&text=rdf-store-idb)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-idb&title=rdf-store-idb)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-idb&t=rdf-store-idb)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-idb)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-idb)

[feature flags]: https://github.com/rust-rdf/rdf.rs/blob/master/lib/rdf-store-idb/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[IndexedDB]: https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API
[RDF]: https://www.w3.org/TR/rdf12-concepts/
[RDF.rs]: https://github.com/rust-rdf/rdf.rs
[Rust]: https://rust-lang.org
[SQLite]: https://sqlite.org
[SPARQL]: https://www.w3.org/TR/sparql12-query/
[sparql-store]: https://github.com/rust-rdf/sparql.rs#readme

[`IdbStore`]: https://docs.rs/rdf-store-idb/latest/rdf_store_idb/struct.IdbStore.html
[`SparqlStore`]: https://docs.rs/sparql-store/latest/sparql_store/struct.SparqlStore.html
