// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use rdf_model::*;
//! ```

#![no_std]
#![deny(unsafe_code)]
#![allow(unused_imports)]

#[doc(hidden)]
pub mod prelude;

mod dataset;
pub use dataset::*;

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

mod source;
pub use source::*;

mod statement;
pub use statement::*;

mod term;
pub use term::*;

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

mod providers {
    #[cfg(feature = "oxrdf")]
    mod oxrdf;
    #[cfg(feature = "oxrdf")]
    pub use oxrdf::*;
    #[cfg(feature = "sophia")]
    mod sophia;
    #[cfg(feature = "sophia")]
    pub use sophia::*;
}

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
