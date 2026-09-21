// This is free and unencumbered software released into the public domain.

//! A Jelly file reader for RDF.rs, a Rust framework for RDF
//! knowledge graphs.
//!
//! # Features
//!
//! This adapter is a scaffold. Runtime dependencies are enabled by `std`;
//! `alloc` alone does not provide a reader implementation.
//!
//! # Examples
//!
//! ```rust
//! use rdf_reader_jelly::*;
//! ```
//!
//! See: <https://jelly-rdf.github.io>

#![no_std]
#![deny(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
