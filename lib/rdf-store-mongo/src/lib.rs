// This is free and unencumbered software released into the public domain.

//! A MongoDB storage adapter for RDF.rs, a Rust framework for RDF
//! knowledge graphs.
//!
//! # Examples
//!
//! ```rust,no_run
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use rdf_model::SAMPLE_QUAD;
//! use rdf_store::{Store, WriteTransaction};
//! use rdf_store_mongo::{MongoStore, MongoTransaction};
//!
//! let mut store = MongoStore::open("mongodb://localhost:27017/test").await?;
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

mod triple;
pub use triple::*;

mod triple_id;
pub use triple_id::*;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
