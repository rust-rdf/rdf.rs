// This is free and unencumbered software released into the public domain.

//! RDF.rs is a Rust framework for working with RDF knowledge graphs.
//!
//! # Examples
//!
//! ```rust
//! use rdf_hash::{TermHash, TripleHash};
//! ```

#![no_std]
#![deny(unsafe_code)]
#![allow(unused_imports)]
#![cfg_attr(docsrs, feature(doc_cfg))]

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
