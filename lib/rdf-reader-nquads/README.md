# RDF.rs Reader: rdf-reader-nquads

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/rdf-reader-nquads)](https://crates.io/crates/rdf-reader-nquads)
[![Documentation](https://docs.rs/rdf-reader-nquads/badge.svg)](https://docs.rs/rdf-reader-nquads)

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

- Supports JSON-LD, Turtle/TriG, N-Triples/N-Quads, COTTAS, and more.
- Built on async Rust using lazily-evaluated streams throughout.
- Plays nice with others: interoperates with Oxigraph, Rudof, and Sophia.
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
cargo add rdf-reader-nquads
```

### Installation in `Cargo.toml`

Enable all default features:

```toml
[dependencies]
rdf-reader-nquads = { version = "0.4" }
```

Enable only specific features:

```toml
[dependencies]
rdf-reader-nquads = { version = "0.4", default-features = false, features = ["serde"] }
```

## 👉 Examples

### Importing the Library

```rust
use rdf_reader_nquads::*;
```

## 📚 Reference

[docs.rs/rdf-reader-nquads](https://docs.rs/rdf-reader-nquads)

### Feature Flags

#### Interoperability

| Feature      | Version | Summary |
| :----------- | :------ | :------ |
| `serde`      | 1.0     | Derives `serde::{Serialize, Deserialize}`

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

[feature flags]: https://github.com/rust-rdf/rdf.rs/blob/master/lib/rdf-reader-nquads/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[RDF]: https://www.w3.org/TR/rdf12-concepts/
[Rust]: https://rust-lang.org
