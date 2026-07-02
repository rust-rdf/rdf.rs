// This is free and unencumbered software released into the public domain.

/// A value of the `xsd:duration` datatype.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#duration>
#[cfg(feature = "jiff")]
pub use jiff::SignedDuration as Duration;

/// A value of the `xsd:duration` datatype.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#duration>
#[cfg(not(feature = "jiff"))]
pub use core::time::Duration;
