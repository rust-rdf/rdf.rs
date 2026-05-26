# XSD.rs: XML Schema for Rust

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.81%2B-blue)](https://rust-lang.org)
[![Package](https://img.shields.io/crates/v/xsd)](https://crates.io/crates/xsd)

🚧 _This is presently under heavy construction._

## ✨ Features

- Supports `no_std` environments from the get-go.
- 100% pure and safe Rust with minimal dependencies and no bloat.
- 100% free and unencumbered public domain software.
- Adheres to the Rust API Guidelines in its [naming conventions].

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.81+

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add xsd
```

## 👉 Examples

### Importing the library

```rust
use xsd::{Type, Value};
use xsd::primitives::{Boolean, Date, DateTime, Decimal, Double, Duration, Float, Time};
```

### Parsing XSD literals

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

### Constructing XSD values

```rust
let value: xsd::Value = "Hello, world!".into();
let value: xsd::Value = true.into();
let value: xsd::Value = 3.1415.into();
let value: xsd::Value = 42.into();
```

### Matching XSD values

```rust
use xsd::Value::*;

let value = xsd::parse("3.1415", xsd::DOUBLE).unwrap();

match value {
    Decimal(value) => eprintln!("{:?}", value),
    Primitive(value) => eprintln!("{:?}", value),
}
```

### Matching XSD decimals

```rust
use xsd::DecimalValue::*;

let value = xsd::parse("3.1415", xsd::DECIMAL).unwrap();

match value.as_decimal() {
    Decimal(_) => eprintln!("it's an xsd::decimal"),
    Integer(_) => eprintln!("it's an xsd::integer"),
    Long(_) => eprintln!("it's an xsd::long"),
    Int(_) => eprintln!("it's an xsd::int"),
    Short(_) => eprintln!("it's an xsd::short"),
    Byte(_) => eprintln!("it's an xsd::byte"),
    _ => unreachable!(),
}
```

## 📚 Reference

https://docs.rs/xsd

## 👨‍💻 Development

```bash
git clone https://github.com/rust-rdf/rdf.rs.git
```

- - -

[![Share on Twitter](https://img.shields.io/badge/share%20on-twitter-03A9F4?logo=twitter)](https://twitter.com/share?url=https://github.com/rust-rdf/rdf.rs/tree/master/lib/xsd&text=XSD.rs)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/rust-rdf/rdf.rs/tree/master/lib/xsd&title=XSD.rs)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hacker%20news-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/rust-rdf/rdf.rs/tree/master/lib/xsd&t=XSD.rs)
[![Share on Facebook](https://img.shields.io/badge/share%20on-facebook-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/rust-rdf/rdf.rs/tree/master/lib/xsd)

[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html
