// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use xsd::*;
//! ```

#![no_std]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub const BASE_URI: &str = "http://www.w3.org/2001/XMLSchema#";

#[allow(unused_imports)]
pub mod primitives {
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

mod decimal_type;
pub use decimal_type::*;

mod decimal_value;
pub use decimal_value::*;

mod parse;
pub use parse::*;

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
