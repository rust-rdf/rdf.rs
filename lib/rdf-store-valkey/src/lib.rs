// This is free and unencumbered software released into the public domain.

//! A Valkey storage adapter for RDF.rs, a Rust framework for RDF
//! knowledge graphs.
//!
//! ```rust
//! use rdf_store_valkey::{ValkeyStore, ValkeyTransaction};
//! ```

#![no_std]
#![deny(unsafe_code)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub use rdf_store::{ReadTransaction, Store, StoreOptions, WriteTransaction};

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

mod triple_id;
pub use triple_id::*;

mod triple_key;
pub use triple_key::*;

mod triple_pattern;
pub use triple_pattern::*;
