# RDF.rs Reader: COTTAS

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/rdf-reader-cottas)](https://crates.io/crates/rdf-reader-cottas)
[![Documentation](https://docs.rs/rdf-reader-cottas/badge.svg)](https://docs.rs/rdf-reader-cottas)

A COTTAS file reader for [RDF.rs] knowledge graphs.

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
cargo add rdf-reader-cottas
```

### Installation in `Cargo.toml`

Enable all default features:

```toml
[dependencies]
rdf-reader-cottas = { version = "0.4" }
```

Enable only specific features:

```toml
[dependencies]
rdf-reader-cottas = { version = "0.4", default-features = false, features = ["serde"] }
```

## 👉 Examples

### Importing the Library

```rust
use rdf_reader_cottas::CottasReader;
```

## 📚 Reference

[docs.rs/rdf-reader-cottas](https://docs.rs/rdf-reader-cottas)

### Feature Flags

#### Interoperability

| Feature      | Version | Summary |
| :----------- | :------ | :------ |
| `serde`      | 1.0     | Derives `serde::{Serialize, Deserialize}`

### See Also

| Package | Crate | Docs |
| :------ | :---- | :--- |
| [rdf-reader-cottas](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-reader-cottas#readme) | [![Package](https://img.shields.io/crates/v/rdf-reader-cottas)](https://crates.io/crates/rdf-reader-cottas) | [![Documentation](https://img.shields.io/docsrs/rdf-reader-cottas?label=docs.rs)](https://docs.rs/rdf-reader-cottas) |
| [rdf-reader-hdt](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-reader-hdt#readme) | [![Package](https://img.shields.io/crates/v/rdf-reader-hdt)](https://crates.io/crates/rdf-reader-hdt) | [![Documentation](https://img.shields.io/docsrs/rdf-reader-hdt?label=docs.rs)](https://docs.rs/rdf-reader-hdt) |
| [rdf-reader-jelly](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-reader-jelly#readme) | [![Package](https://img.shields.io/crates/v/rdf-reader-jelly)](https://crates.io/crates/rdf-reader-jelly) | [![Documentation](https://img.shields.io/docsrs/rdf-reader-jelly?label=docs.rs)](https://docs.rs/rdf-reader-jelly) |
| [rdf-reader-jsonld](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-reader-jsonld#readme) | [![Package](https://img.shields.io/crates/v/rdf-reader-jsonld)](https://crates.io/crates/rdf-reader-jsonld) | [![Documentation](https://img.shields.io/docsrs/rdf-reader-jsonld?label=docs.rs)](https://docs.rs/rdf-reader-jsonld) |
| [rdf-reader-nquads](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-reader-nquads#readme) | [![Package](https://img.shields.io/crates/v/rdf-reader-nquads)](https://crates.io/crates/rdf-reader-nquads) | [![Documentation](https://img.shields.io/docsrs/rdf-reader-nquads?label=docs.rs)](https://docs.rs/rdf-reader-nquads) |
| [rdf-reader-ntriples](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-reader-ntriples#readme) | [![Package](https://img.shields.io/crates/v/rdf-reader-ntriples)](https://crates.io/crates/rdf-reader-ntriples) | [![Documentation](https://img.shields.io/docsrs/rdf-reader-ntriples?label=docs.rs)](https://docs.rs/rdf-reader-ntriples) |
| [rdf-reader-rdfxml](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-reader-rdfxml#readme) | [![Package](https://img.shields.io/crates/v/rdf-reader-rdfxml)](https://crates.io/crates/rdf-reader-rdfxml) | [![Documentation](https://img.shields.io/docsrs/rdf-reader-rdfxml?label=docs.rs)](https://docs.rs/rdf-reader-rdfxml) |
| [rdf-reader-trig](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-reader-trig#readme) | [![Package](https://img.shields.io/crates/v/rdf-reader-trig)](https://crates.io/crates/rdf-reader-trig) | [![Documentation](https://img.shields.io/docsrs/rdf-reader-trig?label=docs.rs)](https://docs.rs/rdf-reader-trig) |
| [rdf-reader-turtle](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-reader-turtle#readme) | [![Package](https://img.shields.io/crates/v/rdf-reader-turtle)](https://crates.io/crates/rdf-reader-turtle) | [![Documentation](https://img.shields.io/docsrs/rdf-reader-turtle?label=docs.rs)](https://docs.rs/rdf-reader-turtle) |

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

[feature flags]: https://github.com/rust-rdf/rdf.rs/blob/master/lib/rdf-reader-cottas/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[RDF]: https://www.w3.org/TR/rdf12-concepts/
[RDF.rs]: https://github.com/rust-rdf/rdf.rs
[Rust]: https://rust-lang.org
