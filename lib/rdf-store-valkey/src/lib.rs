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

mod store;
pub use store::*;

mod transaction;
pub use transaction::*;
