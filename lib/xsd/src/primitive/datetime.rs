// This is free and unencumbered software released into the public domain.

/// A value of the `xsd:dateTime` datatype.
///
/// Requires `jiff` (enabled by `datetime`). This is Jiff's civil dateTime, with
/// years `-9999..=9999`, proleptic Gregorian calendar validation, and nanosecond
/// precision. Year zero is the year before year one; no timezone is stored.
/// Use [`crate::parse_datetime`] for XSD lexical validation and [`crate::Value`]
/// for XSD formatting. The re-export's own parser and formatter follow Jiff's
/// rules, including different negative-year spellings.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#dateTime>
#[cfg(feature = "jiff")]
pub use jiff::civil::DateTime;
