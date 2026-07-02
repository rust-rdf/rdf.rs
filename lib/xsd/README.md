# XSD.rs: XML Schema for Rust

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/xsd)](https://crates.io/crates/xsd)
[![Documentation](https://docs.rs/xsd/badge.svg)](https://docs.rs/xsd)

**XSD.rs** is a [Rust] implementation of the [XML Schema] datatypes.

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
cargo add xsd
```

### Installation in `Cargo.toml`

Enable all default features:

```toml
[dependencies]
xsd = { version = "0.4" }
```

Enable only specific features:

```toml
[dependencies]
xsd = { version = "0.4", default-features = false, features = ["alloc"] }
```

## 👉 Examples

### Importing the Library

```rust
use xsd::{Type, Value};
use xsd::primitive::{Boolean, Date, DateTime, Decimal, Double, Duration, Float, Time};
```

### Parsing XSD Literals

```rust
# fn main() -> Result<(), Box<dyn core::error::Error>> {
let value = xsd::parse("Hello, world!", xsd::STRING)?;
let value = xsd::parse("true", xsd::BOOLEAN)?;
let value = xsd::parse("3.1415", xsd::DOUBLE)?;
let value = xsd::parse("42", xsd::INT)?;
let value = xsd::parse("2026-12-31", xsd::DATE)?;
let value = xsd::parse("2026-12-31T12:34:56", xsd::DATE_TIME)?;
# Ok(())
# }
```

### Constructing XSD Values

```rust
let value: xsd::Value = "Hello, world!".into();
let value: xsd::Value = true.into();
let value: xsd::Value = 3.1415.into();
let value: xsd::Value = 42.into();
```

### Matching XSD Values

```rust
use xsd::Value::*;

let value = xsd::parse("3.1415", xsd::DOUBLE).unwrap();

match value {
    Decimal(value) => eprintln!("{:?}", value),
    Primitive(value) => eprintln!("{:?}", value),
}
```

### Matching XSD Decimals

```rust
use xsd::DecimalValue::*;

let value = xsd::parse("3.1415", xsd::DECIMAL).unwrap();

match value.as_decimal() {
    Decimal(_) => eprintln!("it's an xsd:decimal"),
    Integer(_) => eprintln!("it's an xsd:integer"),
    Long(_) => eprintln!("it's an xsd:long"),
    Int(_) => eprintln!("it's an xsd:int"),
    Short(_) => eprintln!("it's an xsd:short"),
    Byte(_) => eprintln!("it's an xsd:byte"),
    _ => unreachable!(),
}
```

## 📚 Reference

[docs.rs/xsd](https://docs.rs/xsd)

### Feature Flags

#### Interoperability

| Feature      | Version | Summary |
| :----------- | :------ | :------ |
| `borsh`      | 1.6     | Derives `borsh::{BorshSerialize, BorshDeserialize}`
| `bson`       | 3.1     | Implements `From<T> for bson::Bson`
| `jiff`       | 0.2     | Implements `From<T> for jiff::T`, `From<jiff::T> for T`
| `oxrdf`      | 0.3     | Implements `From<T> for oxrdf::T`, `From<oxrdf::T> for T`
| `rudof`      | 0.3     | Planned interop with `rudof_rdf`
| `serde`      | 1.0     | Derives `serde::{Serialize, Deserialize}`
| `sophia`     | 0.10    | Implements `From<T> for sophia::T`, `From<sophia::T> for T`

## 👨‍💻 Development

```bash
git clone https://github.com/rust-rdf/rdf.rs.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/rust-rdf/rdf.rs/tree/master/lib/xsd&text=XSD.rs)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/rust-rdf/rdf.rs/tree/master/lib/xsd&title=XSD.rs)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/rust-rdf/rdf.rs/tree/master/lib/xsd&t=XSD.rs)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/rust-rdf/rdf.rs/tree/master/lib/xsd)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/rust-rdf/rdf.rs/tree/master/lib/xsd)

[feature flags]: https://github.com/rust-rdf/rdf.rs/blob/master/lib/xsd/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[Rust]: https://rust-lang.org
[XML Schema]: http://www.w3.org/TR/xmlschema-2/
