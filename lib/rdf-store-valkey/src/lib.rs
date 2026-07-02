// This is free and unencumbered software released into the public domain.

//! A Valkey (fka Redis) storage adapter for RDF.rs, a Rust framework for RDF
//! knowledge graphs.
//!
//! # Examples
//!
//! ```rust,no_run
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use rdf_model::SAMPLE_QUAD;
//! use rdf_store::{Store, WriteTransaction};
//! use rdf_store_valkey::{ValkeyStore, ValkeyTransaction};
//!
//! let mut store = ValkeyStore::open("redis://localhost:6379")?;
//!
//! let mut tx = store.write().await?;
//!
//! tx.insert(SAMPLE_QUAD).await?;
//!
//! tx.commit().await?;
//! # Ok(())
//! # }
//! ```

#![no_std]
#![deny(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

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

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
