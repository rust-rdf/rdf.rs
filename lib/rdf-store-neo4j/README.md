# RDF.rs: Neo4j Store

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/rdf-store-neo4j)](https://crates.io/crates/rdf-store-neo4j)
[![Documentation](https://docs.rs/rdf-store-neo4j/badge.svg)](https://docs.rs/rdf-store-neo4j)

A [Neo4j] storage adapter for **[RDF.rs]** knowledge graphs.

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

- Implements a scalable, high-performance RDF store backed by [Neo4j].
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
cargo add rdf-store-neo4j
```

### Installation in `Cargo.toml`

Enable all default features:

```toml
[dependencies]
rdf-store-neo4j = { version = "0.3" }
```

Enable only specific features:

```toml
[dependencies]
rdf-store-neo4j = { version = "0.3", default-features = false, features = ["tls"] }
```

## 👉 Examples

### Running a Neo4j Server

```bash
docker run -p 7474:7474 -p 7687:7687 -e NEO4J_AUTH=neo4j/your_password neo4j
```

### Importing the Library

```rust
use rdf_store_neo4j::{Neo4jStore, Neo4jTransaction};
```

## 📚 Reference

[docs.rs/rdf-store-neo4j](https://docs.rs/rdf-store-neo4j)

## 👨‍💻 Development

```bash
git clone https://github.com/rust-rdf/rdf.rs.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-neo4j&text=rdf-store-neo4j)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-neo4j&title=rdf-store-neo4j)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-neo4j&t=rdf-store-neo4j)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-neo4j)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-neo4j)

[feature flags]: https://github.com/rust-rdf/rdf.rs/blob/master/lib/rdf-store-neo4j/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[Neo4j]: https://neo4j.com
[RDF]: https://www.w3.org/TR/rdf12-concepts/
[RDF.rs]: https://github.com/rust-rdf/rdf.rs
[Rust]: https://rust-lang.org
