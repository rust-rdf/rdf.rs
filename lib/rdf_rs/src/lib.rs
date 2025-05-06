// This is free and unencumbered software released into the public domain.

//! ```rust,compile_fail
//! # use rdf::*;
//! ```

pub use rdf_derive as derive;
pub use rdf_format as format;
pub use rdf_model as model;
pub use rdf_query as query;
pub use rdf_reader as reader;
pub use rdf_vocab as vocab;
pub use rdf_writer as writer;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
