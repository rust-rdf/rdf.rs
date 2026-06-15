// This is free and unencumbered software released into the public domain.

//! A TriG file reader for RDF.rs, a Rust framework for RDF
//! knowledge graphs.
//!
//! # Examples
//!
//! ```rust
//! use rdf_reader_trig::TrigReader;
//! ```

#![no_std]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod error;
pub use error::*;

mod reader;
pub use reader::*;

mod triple;
pub use triple::*;
