// This is free and unencumbered software released into the public domain.

//! RDF.rs is a Rust framework for working with RDF knowledge graphs.
//!
//! ```rust
//! use rdf_query::*;
//! ```

#![no_std]
#![deny(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
