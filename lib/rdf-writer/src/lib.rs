// This is free and unencumbered software released into the public domain.

//! RDF.rs is a Rust framework for working with RDF knowledge graphs.
//!
//! # Examples
//!
//! ```rust
//! use rdf_writer::*;
//! ```

#![no_std]
#![deny(unsafe_code)]
#![allow(unused_imports)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod writer;
pub use writer::*;

mod writer_options;
pub use writer_options::*;

/// Interoperability with other Rust libraries.
pub mod interop {
    #[cfg(feature = "oxrdf")]
    mod oxrdf;
    #[cfg(feature = "oxrdf")]
    pub use oxrdf::*;

    #[cfg(feature = "sophia")]
    mod sophia;
    #[cfg(feature = "sophia")]
    pub use sophia::*;
}
