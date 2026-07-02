// This is free and unencumbered software released into the public domain.

//! RDF.rs is a Rust framework for working with RDF knowledge graphs.
//!
//! ```rust
//! use rdf_vocab::{rdf, rdfs, xsd};
//! ```

#![no_std]
#![deny(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod rdf;
pub mod rdfs;
pub mod xsd;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
