// This is free and unencumbered software released into the public domain.

//! XML Schema (XSD) datatypes for Rust.
//!
//! # Features
//!
//! Disable defaults for `no_std`. Numeric values and datatype identifiers are
//! available without this crate's `alloc` feature; `alloc` enables owned strings
//! and allocation-dependent helpers. `datetime` (also exposed as `jiff`) enables
//! date/time values without requiring `std`. `serde` and `borsh` imply `alloc`;
//! `bson`, `oxrdf`, and `rudof` require `std`. Defaults enable `all` and `std`.
//! Dependency features are forwarded explicitly; enabling serialization does
//! not implicitly enable date/time support.
//!
//! ```rust
//! use xsd::{Type, Value};
//! use xsd::primitive::{Boolean, Decimal, Double, Duration, Float};
//! # #[cfg(feature = "jiff")]
//! use xsd::primitive::{Date, DateTime, Time};
//! ```
//!
//! ## Sophia interoperability
//!
//! The `sophia` feature is currently an empty compatibility placeholder. It
//! enables no dependencies, trait implementations, or conversion APIs in this
//! crate, and does not enable `alloc` or `std`. In particular, enabling it does
//! not make [`Value`] or [`PrimitiveValue`] implement Sophia term traits.
//! Applications integrating with Sophia must provide their own term conversion,
//! retaining RDF lexical forms and datatype identifiers outside parsed values.
//!
//! ## JSON-LD interoperability
//!
//! The `json-ld` feature is currently an empty compatibility placeholder. It
//! enables no dependencies, JSON-LD processing, or conversion APIs, and does not
//! enable `alloc`, `std`, or `serde`. Explicit JSON conversion methods on values
//! require the separate `serde` feature. Those methods produce JSON values,
//! not JSON-LD value objects carrying `@value` and `@type`; they do not provide
//! JSON-LD expansion, compaction, or RDF datatype-preserving round trips.
//!
//! ## Rudof interoperability
//!
//! The `rudof` feature enables `std` (and thus `alloc`) and the optional
//! `oxsdatatypes` dependency. It currently provides no dedicated Rudof or
//! `oxsdatatypes` conversion API, trait implementations, or public re-exports.
//! It also does not enable `datetime` or change which datatypes [`parse`]
//! supports. Applications using `oxsdatatypes` directly must declare their own
//! dependency and handle conversion and representation limits explicitly.
//!
//! ## Borsh interoperability
//!
//! The `borsh` feature enables `BorshSerialize` and `BorshDeserialize` for
//! [`Type`], [`PrimitiveType`], [`DecimalType`], [`DecimalValue`], and
//! [`TimezoneOffset`]. It also
//! forwards Borsh support to the numeric backend and enables `alloc`, without
//! requiring `std`.
//!
//! [`Value`] and [`PrimitiveValue`] do **not** implement Borsh serialization or
//! deserialization, even with `borsh` enabled. Wrapping a supported numeric
//! value in either enum does not make that wrapper serializable.
//!
//! The supported enums use structural binary encodings, not XSD lexical strings.
//! Round trips preserve their variants and stored data; they cannot recover
//! original lexical spelling that was discarded during parsing. This feature
//! does not define a versioned, cross-release interchange format.

#![cfg_attr(
    feature = "borsh",
    doc = r#"
### Borsh examples

Supported types round-trip through Borsh (requires `borsh`):

```
fn round_trip<T>(value: T)
where
    T: borsh::BorshSerialize + borsh::BorshDeserialize + PartialEq + core::fmt::Debug,
{
    let bytes = borsh::to_vec(&value).unwrap();
    assert_eq!(borsh::from_slice::<T>(&bytes).unwrap(), value);
}
round_trip(xsd::DATE);
round_trip(xsd::PrimitiveType::Date);
round_trip(xsd::DecimalType::Int);
round_trip(xsd::DecimalValue::Int(42));
```

The general value wrapper is intentionally outside this supported boundary:

```compile_fail,E0277
let value = xsd::Value::from(xsd::DecimalValue::Int(42));
let bytes = borsh::to_vec(&value).unwrap();
```

The primitive wrapper likewise does not implement Borsh deserialization:

```compile_fail,E0277
let value = borsh::from_slice::<xsd::PrimitiveValue>(&[]).unwrap();
```
"#
)]
#![no_std]
#![deny(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

/// The XSD namespace base URI (`http://www.w3.org/2001/XMLSchema#`).
pub const BASE_URI: &'static str = "http://www.w3.org/2001/XMLSchema#";

/// Rust types for representing values of XSD primitive datatypes.
#[allow(unused_imports)]
pub mod primitive {
    mod boolean;
    pub use boolean::*;

    mod date;
    pub use date::*;

    mod datetime;
    pub use datetime::*;

    mod decimal;
    pub use decimal::*;

    mod double;
    pub use double::*;

    mod duration;
    pub use duration::*;

    mod float;
    pub use float::*;

    mod gday;
    pub use gday::*;

    mod gmonth;
    pub use gmonth::*;

    mod gmonthday;
    pub use gmonthday::*;

    mod gyear;
    pub use gyear::*;

    mod gyearmonth;
    pub use gyearmonth::*;

    mod string;
    pub use string::*;

    mod time;
    pub use time::*;
}

/// Rust types for representing values of XSD derived datatypes.
pub mod derived {
    mod integer;
    pub use integer::*;
}

mod decimal_type;
pub use decimal_type::*;

mod decimal_value;
pub use decimal_value::*;

mod parse;
pub use parse::*;

mod parse_calendar;
pub use parse_calendar::*;

mod parse_error;
pub use parse_error::*;

mod timezone_offset;
pub use timezone_offset::*;

mod primitive_type;
pub use primitive_type::*;

mod primitive_value;
pub use primitive_value::*;

mod r#type;
pub use r#type::*;

mod value;
pub use value::*;

#[doc = include_str!("../README.md")]
#[cfg(all(doctest, feature = "alloc", feature = "jiff"))]
pub struct ReadmeDoctests;
