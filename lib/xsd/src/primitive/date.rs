// This is free and unencumbered software released into the public domain.

/// A value of the `xsd:date` datatype.
///
/// Requires `jiff` (enabled by `datetime`). This is Jiff's civil date, with years
/// `-9999..=9999` and proleptic Gregorian leap-day validation. Year zero is the
/// year before year one; dates have no timezone field.
/// Use [`crate::parse_date`] for XSD lexical validation and [`crate::Value`] for
/// XSD formatting. The re-export's own parser and formatter follow Jiff's rules,
/// including different negative-year spellings.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#date>
#[cfg(feature = "jiff")]
pub use jiff::civil::Date;
