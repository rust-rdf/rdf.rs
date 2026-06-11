// This is free and unencumbered software released into the public domain.

//! An Oxigraph storage adapter for RDF.rs, a Rust framework for RDF
//! knowledge graphs.
//!
//! # Examples
//!
//! ```rust
//! use rdf_store_oxigraph::{OxigraphStore, OxigraphTransaction};
//! ```

#![no_std]
#![deny(unsafe_code)]

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

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
