// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use xsd::*;
//! ```

#![no_std]
#![deny(unsafe_code)]

pub type Type = PrimitiveType;
pub type Value = PrimitiveValue;

mod primitive_type;
pub use primitive_type::*;

mod primitive_value;
pub use primitive_value::*;
