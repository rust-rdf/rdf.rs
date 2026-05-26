# RDF.rs: Valkey Store

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/rdf-store-valkey)](https://crates.io/crates/rdf-store-valkey)
[![Documentation](https://docs.rs/rdf-store-valkey/badge.svg)](https://docs.rs/rdf-store-valkey)

**RDF.rs** is a [Rust] framework for working with [RDF] knowledge graphs.

🚧 _This is presently under heavy construction._

## ✨ Features

- 100% pure and safe Rust with minimal dependencies and no bloat.
- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- 100% free and unencumbered public domain software.

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
rdf-store-valkey = { version = "0.3", default-features = false, features = ["alloc"] }
```

## 👉 Examples

### Importing the library

```rust
use rdf_store_valkey::{ValkeyStore, ValkeyTransaction};
```

## 📚 Reference

[docs.rs/rdf-store-valkey](https://docs.rs/rdf-store-valkey)

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
[Rust]: https://rust-lang.org
