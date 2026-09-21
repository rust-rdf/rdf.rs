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

mod parse_error;
pub use parse_error::*;

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
