// This is free and unencumbered software released into the public domain.

//! RDF.rs is a Rust framework for working with RDF knowledge graphs.
//!
//! # Features
//!
//! Options and format identifiers are available without defaults. `alloc`
//! enables the reader/source abstraction. `std` enables `StreamIter` and its
//! Tokio runtime dependency; consume this blocking bridge off runtime workers.
//! `serde` enables model serialization with `alloc`; `oxrdf` and `sophia`
//! require `std`. Format adapters have their own backend/runtime requirements.
//!
//! # Examples
//!
//! ```rust
//! use rdf_reader::*;
//! ```

#![no_std]
#![deny(unsafe_code)]
#![allow(unused_imports)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
mod reader;
#[cfg(feature = "alloc")]
pub use reader::*;

mod reader_options;
pub use reader_options::*;

#[cfg(feature = "std")]
mod stream_iter;
#[cfg(feature = "std")]
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

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
