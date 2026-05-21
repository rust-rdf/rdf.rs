// This is free and unencumbered software released into the public domain.

#[cfg(feature = "jiff")]
pub use jiff::SignedDuration as Duration;

#[cfg(not(feature = "jiff"))]
pub use core::time::Duration;
