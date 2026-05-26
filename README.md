# RDF.rs: RDF for Rust

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/rdf_rs)](https://crates.io/crates/rdf_rs)
[![Documentation](https://docs.rs/rdf_rs/badge.svg)](https://docs.rs/rdf_rs)

**RDF.rs** is a [Rust] framework for working with [RDF] knowledge graphs.

🚧 _This is presently under heavy construction._

## ✨ Features

- 100% pure and safe Rust with minimal dependencies and no bloat.
- Supports `no_std` environments from the get-go.
- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition)

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add rdf_rs --rename rdf
```

### Installation in `Cargo.toml` (with all features enabled)

```toml
[dependencies]
rdf = { package = "rdf_rs", version = "0.3" }
```

### Installation in `Cargo.toml` (with only specific features enabled)

```toml
[dependencies]
rdf = { package = "rdf_rs", version = "0.3", default-features = false, features = ["serde"] }
```

## 👉 Examples

### Importing the library

```rust,compile_fail
use rdf::{format, model, query, reader, store, stream, vocab, writer};
```

## 📚 Reference

[docs.rs/rdf_rs](https://docs.rs/rdf_rs)

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
