# RDF.rs: RDF for Rust

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/rdf_rs)](https://crates.io/crates/rdf_rs)
[![Documentation](https://docs.rs/rdf_rs/badge.svg)](https://docs.rs/rdf_rs)

**RDF.rs** is a [Rust] framework for working with [RDF] knowledge graphs.

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

- 100% pure and safe Rust with minimal dependencies and no bloat.
- Supports `no_std` environments from the get-go.
- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- Cuts red tape: 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition)

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add rdf_rs --rename rdf
```

### Installation in `Cargo.toml`

Enable all default features:

```toml
[dependencies]
rdf = { package = "rdf_rs", version = "0.3" }
```

Enable only specific features:

```toml
[dependencies]
rdf = { package = "rdf_rs", version = "0.3", default-features = false, features = ["serde"] }
```

## 👉 Examples

### Importing the Library

```rust,compile_fail
use rdf::{format, hash, id, model, query, reader, store, stream, vocab, writer};
```

## 📚 Reference

[docs.rs/rdf_rs](https://docs.rs/rdf_rs)

### Core Packages

| Package | Crate | Docs |
| :------ | :---- | :--- |
| [rdf](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf_rs#readme) | [![Package](https://img.shields.io/crates/v/rdf_rs)](https://crates.io/crates/rdf_rs) | [![Documentation](https://img.shields.io/docsrs/rdf_rs?label=docs.rs)](https://docs.rs/rdf_rs) |
| [rdf-derive](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-derive#readme) | [![Package](https://img.shields.io/crates/v/rdf-derive)](https://crates.io/crates/rdf-derive) | [![Documentation](https://img.shields.io/docsrs/rdf-derive?label=docs.rs)](https://docs.rs/rdf-derive) |
| [rdf-format](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-format#readme) | [![Package](https://img.shields.io/crates/v/rdf-format)](https://crates.io/crates/rdf-format) | [![Documentation](https://img.shields.io/docsrs/rdf-format?label=docs.rs)](https://docs.rs/rdf-format) |
| [rdf-hash](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-hash#readme) | [![Package](https://img.shields.io/crates/v/rdf-hash)](https://crates.io/crates/rdf-hash) | [![Documentation](https://img.shields.io/docsrs/rdf-hash?label=docs.rs)](https://docs.rs/rdf-hash) |
| [rdf-id](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-id#readme) | [![Package](https://img.shields.io/crates/v/rdf-id)](https://crates.io/crates/rdf-id) | [![Documentation](https://img.shields.io/docsrs/rdf-id?label=docs.rs)](https://docs.rs/rdf-id) |
| [rdf-model](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-model#readme) | [![Package](https://img.shields.io/crates/v/rdf-model)](https://crates.io/crates/rdf-model) | [![Documentation](https://img.shields.io/docsrs/rdf-model?label=docs.rs)](https://docs.rs/rdf-model) |
| [rdf-query](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-query#readme) | [![Package](https://img.shields.io/crates/v/rdf-query)](https://crates.io/crates/rdf-query) | [![Documentation](https://img.shields.io/docsrs/rdf-query?label=docs.rs)](https://docs.rs/rdf-query) |
| [rdf-reader](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-reader#readme) | [![Package](https://img.shields.io/crates/v/rdf-reader)](https://crates.io/crates/rdf-reader) | [![Documentation](https://img.shields.io/docsrs/rdf-reader?label=docs.rs)](https://docs.rs/rdf-reader) |
| [rdf-store](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store#readme) | [![Package](https://img.shields.io/crates/v/rdf-store)](https://crates.io/crates/rdf-store) | [![Documentation](https://img.shields.io/docsrs/rdf-store?label=docs.rs)](https://docs.rs/rdf-store) |
| [rdf-stream](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-stream#readme) | [![Package](https://img.shields.io/crates/v/rdf-stream)](https://crates.io/crates/rdf-stream) | [![Documentation](https://img.shields.io/docsrs/rdf-stream?label=docs.rs)](https://docs.rs/rdf-stream) |
| [rdf-vocab](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-vocab#readme) | [![Package](https://img.shields.io/crates/v/rdf-vocab)](https://crates.io/crates/rdf-vocab) | [![Documentation](https://img.shields.io/docsrs/rdf-vocab?label=docs.rs)](https://docs.rs/rdf-vocab) |
| [rdf-writer](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-writer#readme) | [![Package](https://img.shields.io/crates/v/rdf-writer)](https://crates.io/crates/rdf-writer) | [![Documentation](https://img.shields.io/docsrs/rdf-writer?label=docs.rs)](https://docs.rs/rdf-writer) |
| [xsd](https://github.com/rust-rdf/rdf.rs/tree/master/lib/xsd#readme) | [![Package](https://img.shields.io/crates/v/xsd)](https://crates.io/crates/xsd) | [![Documentation](https://img.shields.io/docsrs/xsd?label=docs.rs)](https://docs.rs/xsd) |

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

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/rust-rdf/rdf.rs&text=RDF.rs)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/rust-rdf/rdf.rs&title=RDF.rs)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/rust-rdf/rdf.rs&t=RDF.rs)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/rust-rdf/rdf.rs)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/rust-rdf/rdf.rs)

[feature flags]: https://github.com/rust-rdf/rdf.rs/blob/master/lib/rdf_rs/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[RDF]: https://www.w3.org/TR/rdf12-concepts/
[Rust]: https://rust-lang.org
