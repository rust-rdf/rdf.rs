// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use xsd::*;
//! ```

#![no_std]
#![deny(unsafe_code)]

pub const BASE_URI: &str = "http://www.w3.org/2001/XMLSchema#";

pub type Type = PrimitiveType;
pub type Value = PrimitiveValue;

#[cfg(feature = "jiff")]
pub use jiff::{
    SignedDuration as Duration,
    civil::{Date, DateTime, Time},
};

#[cfg(feature = "rust_decimal")]
pub use rust_decimal::Decimal;

mod primitive_type;
pub use primitive_type::*;

mod primitive_value;
pub use primitive_value::*;
