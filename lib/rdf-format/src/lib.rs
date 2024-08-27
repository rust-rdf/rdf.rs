// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use rdf_format::*;
//! ```

#![no_std]
#![deny(unsafe_code)]

#[doc(hidden)]
pub use rdf_model::prelude;

mod format;
pub use format::*;
