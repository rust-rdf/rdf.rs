# RDF.rs: MongoDB Store

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/rdf-store-mongo)](https://crates.io/crates/rdf-store-mongo)
[![Documentation](https://docs.rs/rdf-store-mongo/badge.svg)](https://docs.rs/rdf-store-mongo)

A [MongoDB] storage adapter for **[RDF.rs]** knowledge graphs.

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

- Implements a scalable, high-performance RDF store backed by [MongoDB].
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
cargo add rdf-store-mongo
```

### Installation in `Cargo.toml`

Enable all default features:

```toml
[dependencies]
rdf-store-mongo = { version = "0.3" }
```

Enable only specific features:

```toml
[dependencies]
rdf-store-mongo = { version = "0.3", default-features = false, features = ["tls"] }
```

## 👉 Examples

### Running a MongoDB Server

```bash
docker run -p 27017:27017 mongodb/mongodb-community-server
```

### Importing the Library

```rust
use rdf_store_mongo::{MongoStore, MongoTransaction};
```

## 📚 Reference

[docs.rs/rdf-store-mongo](https://docs.rs/rdf-store-mongo)

### Further Reading

- [MongoDB Limits and Thresholds](https://www.mongodb.com/docs/manual/reference/limits/)

## 👨‍💻 Development

```bash
git clone https://github.com/rust-rdf/rdf.rs.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-mongo&text=rdf-store-mongo)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-mongo&title=rdf-store-mongo)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-mongo&t=rdf-store-mongo)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-mongo)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-store-mongo)

[feature flags]: https://github.com/rust-rdf/rdf.rs/blob/master/lib/rdf-store-mongo/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[MongoDB]: https://mongodb.org
[RDF]: https://www.w3.org/TR/rdf12-concepts/
[RDF.rs]: https://github.com/rust-rdf/rdf.rs
[Rust]: https://rust-lang.org
