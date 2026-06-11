# RDF.rs: Valkey Store

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
rdf-store-valkey = { version = "0.3" }
```

Enable only specific features:

```toml
[dependencies]
rdf-store-valkey = { version = "0.3", default-features = false, features = ["tls"] }
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

tx.r#match(quad_pattern)
    .for_each(|quad| async move {
        eprintln!("{:?}", quad);
    })
    .await;
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

## 👨‍💻 Development

```bash
git clone https://github.com/rust-rdf/rdf.rs.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-valkey&text=rdf-store-valkey)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-valkey&title=rdf-store-valkey)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-valkey&t=rdf-store-valkey)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-valkey)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-valkey)

[feature flags]: https://github.com/rust-rdf/rdf.rs/blob/master/lib/rdf-store-valkey/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[RDF]: https://www.w3.org/TR/rdf12-concepts/
[RDF.rs]: https://github.com/rust-rdf/rdf.rs
[Rust]: https://rust-lang.org
[Valkey]: https://valkey.io
[Valkey Bundle]: https://valkey.io/topics/valkey-bundle/
[Valkey JSON]: https://valkey.io/topics/valkey-json/
