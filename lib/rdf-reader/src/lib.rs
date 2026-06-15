// This is free and unencumbered software released into the public domain.

//! RDF.rs is a Rust framework for working with RDF knowledge graphs.
//!
//! # Examples
//!
//! ```rust
//! use rdf_reader::*;
//! ```

#![no_std]
#![deny(unsafe_code)]
#![allow(unused_imports)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod reader;
pub use reader::*;

mod reader_options;
pub use reader_options::*;

mod stream_iter;
pub use stream_iter::*;

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
