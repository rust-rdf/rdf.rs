// This is free and unencumbered software released into the public domain.

//! RDF.rs is a Rust framework for working with RDF knowledge graphs.
//!
//! # Feature tiers
//!
//! Disable default features for `no_std`. With no features, generic terms,
//! statements, patterns, and format identifiers remain available. `alloc` adds
//! owned/Cow terms, vocabularies, reader abstractions, and transaction traits;
//! applications must supply an allocator when they allocate. `std` adds the
//! Tokio stream bridge and heap store. Runtime adapters require `std`.
//!
//! `serde`, `borsh`, and `blake3` imply `alloc`, not `std`. `datetime` and
//! `decimal` forward XSD feature selections. `oxrdf`, `bson`, `json-ld`, `rudof`,
//! and `sophia` imply `std` because their dependencies require it. Defaults
//! enable `all` and `std`; `all` includes std-dependent interoperability.
//! These flags select existing APIs/dependencies, including integrations that
//! are still scaffolds.
//!
//! The workspace checks a consuming `#![no_std]` crate on
//! `thumbv7em-none-eabihf`, where `std` is unavailable. Procedural macros run on
//! the host and may use its standard library independently of the target.
//!
//! ```rust
//! use rdf_rs::{format, hash, id, message, model, query, reader, store, stream, vocab, writer};
//! ```

#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use rdf_derive as derive;

pub use rdf_format as format;
pub use rdf_hash as hash;
pub use rdf_id as id;
pub use rdf_message as message;
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
