// This is free and unencumbered software released into the public domain.

/// Rust type for representing values of the `xsd:string` datatype.
#[cfg(feature = "alloc")]
pub use ::alloc::string::String;

/// Rust type for representing values of the `xsd:string` datatype.
#[cfg(not(feature = "alloc"))]
pub type String = &'static str;
