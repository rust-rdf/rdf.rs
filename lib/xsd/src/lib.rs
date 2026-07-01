// This is free and unencumbered software released into the public domain.

//! ```rust
//! use xsd::{Type, Value};
//! use xsd::primitive::{Boolean, Date, DateTime, Decimal, Double, Duration, Float, Time};
//! ```

#![no_std]
#![deny(unsafe_code)]

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

    mod integer;
    pub use integer::*;

    mod string;
    pub use string::*;

    mod time;
    pub use time::*;
}

/// Rust types for representing values of XSD derived datatypes.
pub mod derived {
    // TODO
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
#[cfg(doctest)]
pub struct ReadmeDoctests;
