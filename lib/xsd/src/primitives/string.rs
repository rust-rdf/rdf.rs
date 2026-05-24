// This is free and unencumbered software released into the public domain.

#[cfg(feature = "alloc")]
pub use ::alloc::string::String;

#[cfg(not(feature = "alloc"))]
pub type String = &'static str;
