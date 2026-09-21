// This is free and unencumbered software released into the public domain.

//! RDF.rs is a Rust framework for working with RDF knowledge graphs.
//!
//! # Features
//!
//! With defaults disabled, generic terms, triples/quads, and pattern types need
//! neither this crate's `alloc` nor `std` feature. `alloc` exposes `HeapTerm`,
//! `CowTerm`, datatypes, and boxed iterator/source abstractions. `serde` and
//! `borsh` imply `alloc`; ecosystem integrations (`oxrdf`, `bson`, `json-ld`,
//! `rudof`, `sophia`) imply `std`. `datetime` and `decimal` forward XSD features.
//! The lexical `StatementPattern::matches` helper is available with `alloc`.
//!
//! # Examples
//!
//! ```rust
//! use rdf_model::*;
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
mod cow;
#[cfg(feature = "alloc")]
pub use cow::*;

#[cfg(feature = "alloc")]
mod heap;
#[cfg(feature = "alloc")]
pub use heap::*;

mod any_statement;
pub use any_statement::*;

mod any_term;
pub use any_term::*;

mod base_direction;
pub use base_direction::*;

#[cfg(feature = "alloc")]
mod dataset;
#[cfg(feature = "alloc")]
pub use dataset::*;

#[cfg(feature = "alloc")]
mod datatype;
#[cfg(feature = "alloc")]
pub use datatype::*;

#[cfg(feature = "alloc")]
mod document;
#[cfg(feature = "alloc")]
pub use document::*;

mod feature;
pub use feature::*;

#[cfg(feature = "alloc")]
mod graph;
#[cfg(feature = "alloc")]
pub use graph::*;

mod quad;
pub use quad::*;

mod quad_pattern;
pub use quad_pattern::*;

#[cfg(feature = "alloc")]
mod source;
#[cfg(feature = "alloc")]
pub use source::*;

mod statement;
pub use statement::*;

mod statement_pattern;
pub use statement_pattern::*;

mod statement_slot;
pub use statement_slot::*;

mod term;
pub use term::*;

mod term_kind;
pub use term_kind::*;

mod triple;
pub use triple::*;

mod triple_pattern;
pub use triple_pattern::*;

mod vocabulary;
pub use vocabulary::*;

mod traits {
    mod countable;
    pub use countable::*;

    #[cfg(feature = "alloc")]
    mod enumerable;
    #[cfg(feature = "alloc")]
    pub use enumerable::*;

    mod maybe_durable;
    pub use maybe_durable::*;

    mod maybe_indexed;
    pub use maybe_indexed::*;

    mod maybe_mutable;
    pub use maybe_mutable::*;

    #[cfg(feature = "alloc")]
    mod queryable;
    #[cfg(feature = "alloc")]
    pub use queryable::*;
}
pub use traits::*;

/// Interoperability with other Rust libraries.
pub mod interop {
    //#[cfg(feature = "datafusion")]
    //mod datafusion;
    //#[cfg(feature = "datafusion")]
    //pub use datafusion::*;

    #[cfg(feature = "json-ld")]
    mod json_ld;
    #[cfg(feature = "json-ld")]
    pub use json_ld::*;

    #[cfg(feature = "oxrdf")]
    mod oxrdf;
    #[cfg(feature = "oxrdf")]
    pub use oxrdf::*;

    #[cfg(feature = "rudof")]
    mod rudof;
    #[cfg(feature = "rudof")]
    pub use rudof::*;

    #[cfg(feature = "sophia")]
    mod sophia;
    #[cfg(feature = "sophia")]
    pub use sophia::*;
}

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
