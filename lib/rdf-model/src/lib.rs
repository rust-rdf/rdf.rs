// This is free and unencumbered software released into the public domain.

//! RDF.rs is a Rust framework for working with RDF knowledge graphs.
//!
//! ```rust
//! use rdf_model::*;
//! ```

#![no_std]
#![deny(unsafe_code)]
#![allow(unused_imports)]

extern crate alloc;

/// TBD
#[cfg(feature = "alloc")]
mod cow;
#[cfg(feature = "alloc")]
pub use cow::*;

/// TBD
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

mod dataset;
pub use dataset::*;

mod datatype;
pub use datatype::*;

mod document;
pub use document::*;

mod feature;
pub use feature::*;

mod graph;
pub use graph::*;

mod literal;
pub use literal::*;

mod node;
pub use node::*;

mod quad;
pub use quad::*;

mod quad_pattern;
pub use quad_pattern::*;

mod source;
pub use source::*;

mod statement;
pub use statement::*;

mod statement_pattern;
pub use statement_pattern::*;

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

    mod enumerable;
    pub use enumerable::*;

    mod maybe_durable;
    pub use maybe_durable::*;

    mod maybe_indexed;
    pub use maybe_indexed::*;

    mod maybe_mutable;
    pub use maybe_mutable::*;

    mod queryable;
    pub use queryable::*;
}
pub use traits::*;

pub mod providers {
    #[cfg(feature = "oxrdf")]
    mod oxrdf;
    #[cfg(feature = "oxrdf")]
    pub use oxrdf::*;
    #[cfg(feature = "sophia")]
    mod sophia;
    #[cfg(feature = "sophia")]
    pub use sophia::*;
}
