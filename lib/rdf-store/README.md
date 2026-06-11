# RDF.rs: Heap Store

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/rdf-store)](https://crates.io/crates/rdf-store)
[![Documentation](https://docs.rs/rdf-store/badge.svg)](https://docs.rs/rdf-store)

An in-memory storage adapter for **[RDF.rs]** knowledge graphs.

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

- Implements a scalable, high-performance in-memory RDF quad store.
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
cargo add rdf-store
```

### Installation in `Cargo.toml`

Enable all default features:

```toml
[dependencies]
rdf-store = { version = "0.4" }
```

Enable only specific features:

```toml
[dependencies]
rdf-store = { version = "0.4", default-features = false, features = ["unstable"] }
```

## 👉 Examples

### Importing the Library

```rust
use rdf_store::{HeapStore, HeapTransaction};
```

### Creating a Store

```rust,compile_fail
let mut store = HeapStore::new();
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

[docs.rs/rdf-store](https://docs.rs/rdf-store)

### Core Traits

#### [`Store`](https://docs.rs/rdf-store/latest/rdf_store/trait.Store.html)

```rust,compile_fail
/// A store of statements that supports R/O and R/W transactions.
pub trait Store {
    type Error;
    type Read: ReadTransaction<Error = Self::Error> + Send;
    type Write: WriteTransaction<Error = Self::Error> + Send;

    /// Begins a read-only (R/O) transaction.
    async fn read(&mut self) -> Result<Self::Read, Self::Error>;

    /// Begins a read-write (R/W) transaction.
    async fn write(&mut self) -> Result<Self::Write, Self::Error>;
}
```

#### [`ReadTransaction`](https://docs.rs/rdf-store/latest/rdf_store/trait.ReadTransaction.html)

```rust,compile_fail
/// A read-only (R/O) transaction on a [`Store`].
pub trait ReadTransaction {
    type Error;
    type Term: Term + Clone;
    type Statement: Statement<Term = Self::Term>;
    type StatementPattern: StatementPattern<Term = Self::Term>;

    /// Returns a stream of all context terms (graph names) in the store.
    fn contexts(&self) -> impl Stream<Item = Result<Self::Term, Self::Error>>;

    /// Returns `true` if the store contains the given statement (pattern).
    async fn contains(
        &self,
        pattern: impl Into<Self::StatementPattern>,
    ) -> Result<bool, Self::Error>;

    /// Returns the number of statements matching the given statement pattern.
    async fn count(
        &self,
        pattern: impl Into<Self::StatementPattern>,
    ) -> Result<u64, Self::Error>;

    /// Returns a stream of all statements matching the given statement pattern.
    fn r#match(
        &self,
        pattern: impl Into<Self::StatementPattern>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>>;
}
```

#### [`WriteTransaction`](https://docs.rs/rdf-store/latest/rdf_store/trait.WriteTransaction.html)

```rust,compile_fail
/// A read-write (R/W) transaction on a [`Store`].
pub trait WriteTransaction {
    type Error;
    type Term: Term + Clone;
    type Statement: Statement<Term = Self::Term>;
    type StatementPattern: StatementPattern<Term = Self::Term>;

    /// Aborts the transaction, discarding any changes.
    async fn rollback(self) -> Result<(), Self::Error>;

    /// Commits the transaction, applying all changes.
    async fn commit(self) -> Result<(), Self::Error>;

    /// Clears all data from the store.
    async fn clear(&mut self) -> Result<(), Self::Error>;

    /// Inserts a statement into the store.
    async fn insert(
        &mut self,
        statement: impl Into<Self::Statement> + Send,
    ) -> Result<(), Self::Error>;

    /// Removes a statement from the store.
    async fn remove(
        &mut self,
        statement: impl Into<Self::Statement> + Send,
    ) -> Result<(), Self::Error>;

    /// Deletes all statements matching the given statement pattern.
    async fn delete(
        &mut self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> Result<(), Self::Error>;
}
```

### Storage Adapters

| Package | Crate | Docs |
| :------ | :---- | :--- |
| [rdf-store](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store#readme) | [![Package](https://img.shields.io/crates/v/rdf-store)](https://crates.io/crates/rdf-store) | [![Documentation](https://img.shields.io/docsrs/rdf-store?label=docs.rs)](https://docs.rs/rdf-store) |
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

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store&text=rdf-store)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store&title=rdf-store)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store&t=rdf-store)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store)

[feature flags]: https://github.com/rust-rdf/rdf.rs/blob/master/lib/rdf-store/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[RDF]: https://www.w3.org/TR/rdf12-concepts/
[RDF.rs]: https://github.com/rust-rdf/rdf.rs
[Rust]: https://rust-lang.org
