// This is free and unencumbered software released into the public domain.

//! An RDF/XML file reader for RDF.rs, a Rust framework for RDF
//! knowledge graphs.
//!
//! # Examples
//!
//! ```rust
//! use rdf_reader_rdfxml::RdfxmlReader;
//! ```

#![no_std]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "oxrdf")]
mod oxrdf {
    mod error;
    pub use error::*;

    mod reader;
    pub use reader::*;

    mod triple;
    pub use triple::*;
}
#[cfg(feature = "oxrdf")]
pub use oxrdf::*;
