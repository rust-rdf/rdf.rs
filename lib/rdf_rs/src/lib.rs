// This is free and unencumbered software released into the public domain.

//! RDF.rs is a Rust framework for working with RDF knowledge graphs.
//!
//! ```rust
//! use rdf_rs::{format, hash, id, model, query, reader, store, stream, vocab, writer};
//! ```

#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use rdf_derive as derive;

pub use rdf_format as format;
pub use rdf_hash as hash;
pub use rdf_id as id;
pub use rdf_model as model;
pub use rdf_query as query;
pub use rdf_reader as reader;
pub use rdf_store as store;
pub use rdf_stream as stream;
pub use rdf_vocab as vocab;
pub use rdf_writer as writer;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
