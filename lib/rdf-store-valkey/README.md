# RDF.rs Store: Valkey

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/rdf-store-valkey)](https://crates.io/crates/rdf-store-valkey)
[![Documentation](https://docs.rs/rdf-store-valkey/badge.svg)](https://docs.rs/rdf-store-valkey)

A [Valkey] (fka Redis) storage adapter for **[RDF.rs]** knowledge graphs.

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

- Implements a scalable, high-performance RDF store backed by [Valkey].
- Compatible with [Valkey Bundle] (requires the [Valkey JSON] module).
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
cargo add rdf-store-valkey
```

### Installation in `Cargo.toml`

Enable all default features:

```toml
[dependencies]
rdf-store-valkey = { version = "0.4" }
```

Enable only specific features:

```toml
[dependencies]
rdf-store-valkey = { version = "0.4", default-features = false, features = ["tracing"] }
```

## 👉 Examples

### Running a Valkey Server

```bash
docker run -p 6379:6379 valkey/valkey-bundle
```

### Importing the Library

```rust
use rdf_store_valkey::{ValkeyStore, ValkeyTransaction};
```

### Connecting to the Store

```rust,compile_fail
let mut store = ValkeyStore::open("redis://localhost")?;
```

### Mutating the Store

```rust,compile_fail
let mut tx = store.write().await?;

tx.remove(old_quad).await?;
tx.insert(new_quad).await?;

tx.commit().await?;
```

### Accessing the Store

```rust,compile_fail
let tx = store.read().await?;

let count = tx.count(quad_pattern).await?;

tx.r#match(quad_pattern)
    .for_each(|quad| async move {
        eprintln!("{:?}", quad);
    })
    .await;
```

### Querying the Store with SPARQL

To execute SPARQL queries on the store, use the [sparql-store] crate to wrap
the underlying [`ValkeyStore`] quad store into a [`SparqlStore`]:

```rust,compile_fail
use sparql_store::SparqlStore;

let mut store: SparqlStore<ValkeyStore> =
    ValkeyStore::open("redis://localhost:6379")?.into();

let tx = store.read().await?;
```

## 📚 Reference

[docs.rs/rdf-store-valkey](https://docs.rs/rdf-store-valkey)

### Storage Schema

The current implementation is based on a triple-centric schema where triples
are uniquely identified and deduplicated by their subject-predicate-object
(SPO) hash. Graphs, in turn, are represented as sets of triple IDs.

```mermaid
graph TD
  RG["rdf:g — set of graph IDs"]
  RG --> G["rdf:g:{graph_id} — set of triple IDs"]
  G --> T["rdf:j:{triple_id} — JSON object with s/p/o"]
  T --> S["s: subject term (JSON-LD)"]
  T --> P["p: predicate term (JSON-LD)"]
  T --> O["o: object term (JSON-LD)"]
```

### Sequence Diagrams

#### Insert Statement

```mermaid
sequenceDiagram
  participant Client as Client
  participant Valkey as Valkey Server
  Client->>Valkey: JSON.SET rdf:j:{triple_id} $ {triple-json}
  Client->>Valkey: SADD rdf:g:{graph_id} {triple_id}
  Client->>Valkey: SADD rdf:g {graph_id}
  Valkey-->>Client: QUEUED
```

#### Remove Statement

```mermaid
sequenceDiagram
  participant Client as Client
  participant Valkey as Valkey Server
  Client->>Valkey: SREM rdf:g:{graph_id} {triple_id}
  Valkey-->>Client: QUEUED
```

#### Write Transaction

```mermaid
sequenceDiagram
  participant App as Application
  participant Client as Client (rdf-store-valkey)
  participant Valkey as Valkey Server

  rect transparent
  Note over App,Client: Begin transaction
  App->>Client: store.begin(writable: true)
  Client->>Valkey: MULTI
  Valkey-->>Client: OK
  Client-->>App: tx
  end

  rect transparent
  Note over App,Client: Enqueue mutations
  App->>Client: tx.remove(statement)
  Client->>Valkey: SREM rdf:g:{graph_id} {triple_id}
  Valkey-->>Client: QUEUED
  App->>Client: tx.insert(statement)
  Client->>Valkey: JSON.SET rdf:j:{triple_id} $ {triple-json}
  Client->>Valkey: SADD rdf:g:{graph_id} {triple_id}
  Client->>Valkey: SADD rdf:g {graph_id}
  Valkey-->>Client: QUEUED
  end

  rect transparent
  Note over App,Client: Commit transaction
  App->>Client: tx.commit()
  Client->>Valkey: EXEC
  Valkey-->>Client: RESP array
  Client-->>App: drop(tx)
  end

  rect transparent
  Note over App,Client: Rollback transaction
  App->>Client: tx.rollback()
  Client->>Valkey: DISCARD
  Valkey-->>Client: OK
  Client-->>App: drop(tx)
  end
```

### See Also

| Package | Crate | Docs |
| :------ | :---- | :--- |
| [rdf-store-idb](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-idb#readme) | [![Package](https://img.shields.io/crates/v/rdf-store-idb)](https://crates.io/crates/rdf-store-idb) | [![Documentation](https://img.shields.io/docsrs/rdf-store-idb?label=docs.rs)](https://docs.rs/rdf-store-idb) |
| [rdf-store-mongo](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-mongo#readme) | [![Package](https://img.shields.io/crates/v/rdf-store-mongo)](https://crates.io/crates/rdf-store-mongo) | [![Documentation](https://img.shields.io/docsrs/rdf-store-mongo?label=docs.rs)](https://docs.rs/rdf-store-mongo) |
| [rdf-store-neo4j](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-neo4j#readme) | [![Package](https://img.shields.io/crates/v/rdf-store-neo4j)](https://crates.io/crates/rdf-store-neo4j) | [![Documentation](https://img.shields.io/docsrs/rdf-store-neo4j?label=docs.rs)](https://docs.rs/rdf-store-neo4j) |
| [rdf-store-oxigraph](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-oxigraph#readme) | [![Package](https://img.shields.io/crates/v/rdf-store-oxigraph)](https://crates.io/crates/rdf-store-oxigraph) | [![Documentation](https://img.shields.io/docsrs/rdf-store-oxigraph?label=docs.rs)](https://docs.rs/rdf-store-oxigraph) |
| [rdf-store-postgres](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-postgres#readme) | [![Package](https://img.shields.io/crates/v/rdf-store-postgres)](https://crates.io/crates/rdf-store-postgres) | [![Documentation](https://img.shields.io/docsrs/rdf-store-postgres?label=docs.rs)](https://docs.rs/rdf-store-postgres) |
| [rdf-store-qlever](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-qlever#readme) | [![Package](https://img.shields.io/crates/v/rdf-store-qlever)](https://crates.io/crates/rdf-store-qlever) | [![Documentation](https://img.shields.io/docsrs/rdf-store-qlever?label=docs.rs)](https://docs.rs/rdf-store-qlever) |
| [rdf-store-sqlite](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-sqlite#readme) | [![Package](https://img.shields.io/crates/v/rdf-store-sqlite)](https://crates.io/crates/rdf-store-sqlite) | [![Documentation](https://img.shields.io/docsrs/rdf-store-sqlite?label=docs.rs)](https://docs.rs/rdf-store-sqlite) |
| [rdf-store-turso](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-turso#readme) | [![Package](https://img.shields.io/crates/v/rdf-store-turso)](https://crates.io/crates/rdf-store-turso) | [![Documentation](https://img.shields.io/docsrs/rdf-store-turso?label=docs.rs)](https://docs.rs/rdf-store-turso) |
| [rdf-store-valkey](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-valkey#readme) | [![Package](https://img.shields.io/crates/v/rdf-store-valkey)](https://crates.io/crates/rdf-store-valkey) | [![Documentation](https://img.shields.io/docsrs/rdf-store-valkey?label=docs.rs)](https://docs.rs/rdf-store-valkey) |
| [rdf-store-virtuoso](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-virtuoso#readme) | [![Package](https://img.shields.io/crates/v/rdf-store-virtuoso)](https://crates.io/crates/rdf-store-virtuoso) | [![Documentation](https://img.shields.io/docsrs/rdf-store-virtuoso?label=docs.rs)](https://docs.rs/rdf-store-virtuoso) |

## 👨‍💻 Development

```bash
git clone https://github.com/rust-rdf/rdf.rs.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https%3A%2F%2Fgithub.com%2Frust-rdf%2Frdf.rs&text=RDF.rs)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https%3A%2F%2Fgithub.com%2Frust-rdf%2Frdf.rs&title=RDF.rs)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https%3A%2F%2Fgithub.com%2Frust-rdf%2Frdf.rs&t=RDF.rs)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fgithub.com%2Frust-rdf%2Frdf.rs)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fgithub.com%2Frust-rdf%2Frdf.rs)

[feature flags]: https://github.com/rust-rdf/rdf.rs/blob/master/lib/rdf-store-valkey/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[ACID transactions]: https://www.mongodb.com/docs/manual/core/transactions/
[IndexedDB]: https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API
[MongoDB]: https://mongodb.org
[Neo4j]: https://neo4j.com
[Oxigraph]: https://oxigraph.org
[PostgreSQL]: https://postgresql.org
[QLever]: https://qlever.dev
[RDF]: https://www.w3.org/TR/rdf12-concepts/
[RDF.rs]: https://github.com/rust-rdf/rdf.rs
[Rust]: https://rust-lang.org
[SPARQL]: https://www.w3.org/TR/sparql12-query/
[SQLite]: https://sqlite.org
[Turso]: https://turso.tech
[Valkey]: https://valkey.io
[Valkey Bundle]: https://valkey.io/topics/valkey-bundle/
[Valkey JSON]: https://valkey.io/topics/valkey-json/
[Virtuoso]: https://virtuoso.openlinksw.com
[sparql-store]: https://github.com/rust-rdf/sparql.rs#readme

[`IdbStore`]: https://docs.rs/rdf-store-idb/latest/rdf_store_idb/struct.IdbStore.html
[`MongoStore`]: https://docs.rs/rdf-store-mongo/latest/rdf_store_mongo/struct.MongoStore.html
[`Neo4jStore`]: https://docs.rs/rdf-store-neo4j/latest/rdf_store_neo4j/struct.Neo4jStore.html
[`OxigraphStore`]: https://docs.rs/rdf-store-oxigraph/latest/rdf_store_oxigraph/struct.OxigraphStore.html
[`PostgresStore`]: https://docs.rs/rdf-store-postgres/latest/rdf_store_postgres/struct.PostgresStore.html
[`QleverStore`]: https://docs.rs/rdf-store-qlever/latest/rdf_store_qlever/struct.QleverStore.html
[`SparqlStore`]: https://docs.rs/sparql-store/latest/sparql_store/struct.SparqlStore.html
[`SqliteStore`]: https://docs.rs/rdf-store-sqlite/latest/rdf_store_sqlite/struct.SqliteStore.html
[`TursoStore`]: https://docs.rs/rdf-store-turso/latest/rdf_store_turso/struct.TursoStore.html
[`ValkeyStore`]: https://docs.rs/rdf-store-valkey/latest/rdf_store_valkey/struct.ValkeyStore.html
[`VirtuosoStore`]: https://docs.rs/rdf-store-virtuoso/latest/rdf_store_virtuoso/struct.VirtuosoStore.html
