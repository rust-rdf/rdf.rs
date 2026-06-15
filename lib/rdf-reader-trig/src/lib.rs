// This is free and unencumbered software released into the public domain.

//! A TriG file reader for RDF.rs, a Rust framework for RDF
//! knowledge graphs.
//!
//! # Examples
//!
//! ```rust
//! use rdf_reader_trig::*;
//! ```

#![no_std]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;
