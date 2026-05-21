// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use xsd::*;
//! ```

#![no_std]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub const BASE_URI: &str = "http://www.w3.org/2001/XMLSchema#";

pub type Type = PrimitiveType;
pub type Value = PrimitiveValue;

pub mod primitives {
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

    mod time;
    pub use time::*;
}

mod decimal_type;
pub use decimal_type::*;

mod decimal_value;
pub use decimal_value::*;

mod primitive_type;
pub use primitive_type::*;

mod primitive_value;
pub use primitive_value::*;
