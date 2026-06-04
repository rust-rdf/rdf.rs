// This is free and unencumbered software released into the public domain.

//! RDF.rs is a Rust framework for working with RDF knowledge graphs.
//!
//! ```rust
//! # use rdf_store_valkey::*;
//! ```

#![no_std]
#![deny(unsafe_code)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub use rdf_store::*;

mod error;
pub use error::*;

mod graph_key;
pub use graph_key::*;

mod quad;
pub use quad::*;

mod store;
pub use store::*;

mod term;
pub use term::*;

mod transaction;
pub use transaction::*;

mod triple;
pub use triple::*;

mod triple_key;
pub use triple_key::*;

mod triple_pattern;
pub use triple_pattern::*;
