// This is free and unencumbered software released into the public domain.

//! RDF.rs is a Rust framework for working with RDF knowledge graphs.
//!
//! ```rust
//! use rdf_format::*;
//! ```

#![no_std]
#![deny(unsafe_code)]

#[doc(hidden)]
pub use rdf_model::prelude;

mod format;
pub use format::*;
