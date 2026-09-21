// This is free and unencumbered software released into the public domain.

//! RDF.rs is a Rust framework for working with RDF knowledge graphs.
//!
//! # Examples
//!
//! ```rust
//! # #[cfg(feature = "blake3")]
//! use rdf_hash::{TermHash, TripleHash};
//! ```
//!
//! # Features
//!
//! `blake3` enables hashing and implies `alloc`, but works without `std`.
//! `serde` enables allocating JSON helpers and forwards serialization to an
//! enabled BLAKE3 backend. `serde` and `zeroize` do not implicitly select the
//! backend. Defaults enable `blake3` and `std`.

#![no_std]
#![deny(unsafe_code)]
#![allow(unused_imports)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "blake3")]
mod term_hash;
#[cfg(feature = "blake3")]
pub use term_hash::*;

#[cfg(feature = "blake3")]
mod triple_hash;
#[cfg(feature = "blake3")]
pub use triple_hash::*;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
