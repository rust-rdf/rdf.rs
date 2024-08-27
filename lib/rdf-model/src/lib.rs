// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use rdf_model::*;
//! ```

#![no_std]
#![deny(unsafe_code)]
#![allow(unused)]

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

    mod durable;
    pub use durable::*;

    mod enumerable;
    pub use enumerable::*;

    mod indexable;
    pub use indexable::*;

    mod mutable;
    pub use mutable::*;

    mod queryable;
    pub use queryable::*;
}
pub use traits::*;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
