// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use rdf_model::*;
//! ```

#![no_std]
#![deny(unsafe_code)]
#![allow(unused)]

#[doc(hidden)]
mod prelude;

mod feature;
pub use feature::*;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
