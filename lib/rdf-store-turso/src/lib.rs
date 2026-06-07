// This is free and unencumbered software released into the public domain.

//! A Turso storage adapter for RDF.rs, a Rust framework for RDF
//! knowledge graphs.
//!
//! # Examples
//!
//! ```rust
//! use rdf_store_turso::{TursoStore, TursoTransaction};
//! ```

#![no_std]
#![deny(unsafe_code)]
#![allow(unused_imports)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub use rdf_store::{ReadTransaction, Store, StoreOptions, WriteTransaction};

mod error;
pub use error::*;

mod store;
pub use store::*;

mod transaction;
pub use transaction::*;
