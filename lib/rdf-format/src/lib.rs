// This is free and unencumbered software released into the public domain.

//! RDF.rs is a Rust framework for working with RDF knowledge graphs.
//!
//! Format identifiers, extension lookup, and display work without `alloc` or
//! `std`. The `alloc` feature adds allocated extension lists and the `Named` /
//! `Labeled` implementations. `serde` works without `alloc`; `oxrdf` and `sophia`
//! require `std`. Disable default features when selecting these tiers explicitly.
//!
//! ```rust
//! use rdf_format::*;
//! ```

#![no_std]
#![deny(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod format;
pub use format::*;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
