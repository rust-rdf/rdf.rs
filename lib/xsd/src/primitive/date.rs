// This is free and unencumbered software released into the public domain.

/// A value of the `xsd:date` datatype.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#date>
#[cfg(feature = "jiff")]
pub use jiff::civil::Date;
