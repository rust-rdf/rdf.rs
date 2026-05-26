// This is free and unencumbered software released into the public domain.

/// Rust type for representing values of the `xsd:duration` datatype.
#[cfg(feature = "jiff")]
pub use jiff::SignedDuration as Duration;

/// Rust type for representing values of the `xsd:duration` datatype.
#[cfg(not(feature = "jiff"))]
pub use core::time::Duration;
