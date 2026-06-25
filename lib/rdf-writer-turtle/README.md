# RDF.rs Writer: Turtle

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/rdf-writer-turtle)](https://crates.io/crates/rdf-writer-turtle)
[![Documentation](https://docs.rs/rdf-writer-turtle/badge.svg)](https://docs.rs/rdf-writer-turtle)

A Turtle file writer for [RDF.rs] knowledge graphs.

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
cargo add rdf-writer-turtle
```

### Installation in `Cargo.toml`

Enable all default features:

```toml
[dependencies]
rdf-writer-turtle = { version = "0.4" }
```

Enable only specific features:

```toml
[dependencies]
rdf-writer-turtle = { version = "0.4", default-features = false, features = ["serde"] }
```

## 👉 Examples

### Importing the Library

```rust
use rdf_writer_turtle::TurtleWriter;
```

## 📚 Reference

[docs.rs/rdf-writer-turtle](https://docs.rs/rdf-writer-turtle)

### Feature Flags

#### Interoperability

| Feature      | Version | Summary |
| :----------- | :------ | :------ |
| `serde`      | 1.0     | Derives `serde::{Serialize, Deserialize}`

### See Also

| Package | Crate | Docs |
| :------ | :---- | :--- |
| [rdf-writer-cottas](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-writer-cottas#readme) | [![Package](https://img.shields.io/crates/v/rdf-writer-cottas)](https://crates.io/crates/rdf-writer-cottas) | [![Documentation](https://img.shields.io/docsrs/rdf-writer-cottas?label=docs.rs)](https://docs.rs/rdf-writer-cottas) |
| [rdf-writer-hdt](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-writer-hdt#readme) | [![Package](https://img.shields.io/crates/v/rdf-writer-hdt)](https://crates.io/crates/rdf-writer-hdt) | [![Documentation](https://img.shields.io/docsrs/rdf-writer-hdt?label=docs.rs)](https://docs.rs/rdf-writer-hdt) |
| [rdf-writer-jelly](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-writer-jelly#readme) | [![Package](https://img.shields.io/crates/v/rdf-writer-jelly)](https://crates.io/crates/rdf-writer-jelly) | [![Documentation](https://img.shields.io/docsrs/rdf-writer-jelly?label=docs.rs)](https://docs.rs/rdf-writer-jelly) |
| [rdf-writer-jsonld](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-writer-jsonld#readme) | [![Package](https://img.shields.io/crates/v/rdf-writer-jsonld)](https://crates.io/crates/rdf-writer-jsonld) | [![Documentation](https://img.shields.io/docsrs/rdf-writer-jsonld?label=docs.rs)](https://docs.rs/rdf-writer-jsonld) |
| [rdf-writer-nquads](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-writer-nquads#readme) | [![Package](https://img.shields.io/crates/v/rdf-writer-nquads)](https://crates.io/crates/rdf-writer-nquads) | [![Documentation](https://img.shields.io/docsrs/rdf-writer-nquads?label=docs.rs)](https://docs.rs/rdf-writer-nquads) |
| [rdf-writer-ntriples](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-writer-ntriples#readme) | [![Package](https://img.shields.io/crates/v/rdf-writer-ntriples)](https://crates.io/crates/rdf-writer-ntriples) | [![Documentation](https://img.shields.io/docsrs/rdf-writer-ntriples?label=docs.rs)](https://docs.rs/rdf-writer-ntriples) |
| [rdf-writer-rdfxml](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-writer-rdfxml#readme) | [![Package](https://img.shields.io/crates/v/rdf-writer-rdfxml)](https://crates.io/crates/rdf-writer-rdfxml) | [![Documentation](https://img.shields.io/docsrs/rdf-writer-rdfxml?label=docs.rs)](https://docs.rs/rdf-writer-rdfxml) |
| [rdf-writer-trig](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-writer-trig#readme) | [![Package](https://img.shields.io/crates/v/rdf-writer-trig)](https://crates.io/crates/rdf-writer-trig) | [![Documentation](https://img.shields.io/docsrs/rdf-writer-trig?label=docs.rs)](https://docs.rs/rdf-writer-trig) |
| [rdf-writer-turtle](https://github.com/rust-rdf/rdf.rs/tree/master/lib/rdf-writer-turtle#readme) | [![Package](https://img.shields.io/crates/v/rdf-writer-turtle)](https://crates.io/crates/rdf-writer-turtle) | [![Documentation](https://img.shields.io/docsrs/rdf-writer-turtle?label=docs.rs)](https://docs.rs/rdf-writer-turtle) |

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

[feature flags]: https://github.com/rust-rdf/rdf.rs/blob/master/lib/rdf-writer-turtle/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[RDF]: https://www.w3.org/TR/rdf12-concepts/
[RDF.rs]: https://github.com/rust-rdf/rdf.rs
[Rust]: https://rust-lang.org
