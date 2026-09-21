// This is free and unencumbered software released into the public domain.

//! RDF.rs is a Rust framework for working with RDF knowledge graphs.
//!
//! RDF/RDFS constants use `CowTerm` and require `alloc`, but not `std`.
//! Use `default-features = false, features = ["alloc"]` in a `no_std` consumer.
//!
//! ```rust
//! # #[cfg(feature = "alloc")]
//! use rdf_vocab::{rdf, rdfs, xsd};
//! ```

#![no_std]
#![deny(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
pub mod rdf;
#[cfg(feature = "alloc")]
pub mod rdfs;
pub mod xsd;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
