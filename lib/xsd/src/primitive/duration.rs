// This is free and unencumbered software released into the public domain.

/// A signed, fixed-length subset of `xsd:duration`, backed by Jiff.
///
/// Available with `jiff` (also enabled by `datetime`). Stores seconds and
/// nanoseconds, without calendar-month components. The supported range is
/// `i64::MIN` seconds minus 999,999,999 nanoseconds through `i64::MAX` seconds
/// plus 999,999,999 nanoseconds. This is not the full XSD duration value space.
///
/// Use [`crate::parse_duration`] for XSD lexical validation: parsing this alias
/// directly uses Jiff's more permissive grammar. The XSD parser currently accepts
/// time components (hours, minutes, seconds), but rejects year, month, and day
/// components, including `P1D`. Fractions are limited to nine digits; excess
/// precision and overflow return errors rather than rounding or truncating.
///
/// Formatting produces a normalized time-only duration. Original unit choices,
/// leading zeros, trailing fractional zeros, and the sign of zero are not
/// retained. Equality and ordering compare fixed elapsed durations, not XSD's
/// calendar-relative duration semantics. Backend constructors and arithmetic
/// retain their own documented overflow and panic behavior.
///
/// ```
/// use xsd::primitive::Duration;
/// let value = xsd::Value::from(Duration::new(0, -1));
/// assert_eq!(value.to_string(), "-PT0.000000001S");
/// assert_eq!(xsd::parse_duration(value.to_string()).unwrap(), value);
/// assert!(xsd::parse_duration("P1M").is_err());
/// ```
///
/// See: <https://www.w3.org/TR/xmlschema-2/#duration>
#[cfg(feature = "jiff")]
pub use jiff::SignedDuration as Duration;

/// An unsigned Rust duration alias used when `jiff` is disabled.
///
/// This is [`core::time::Duration`], not a complete XSD duration representation:
/// it cannot store negative durations or calendar-month components. Its range
/// is zero through `u64::MAX` seconds plus 999,999,999 nanoseconds. Its formatting
/// and arithmetic follow the standard-library type, not XSD lexical rules.
///
/// This alias does not enable duration parsing or a duration variant in
/// [`crate::Value`]. [`crate::parse`] returns
/// [`crate::ParseError::UnsupportedDatatype`] for [`crate::DURATION`] in this
/// configuration. Enable `jiff` (or `datetime`) for the signed representation
/// and the XSD parser.
///
/// ```
/// let duration = xsd::primitive::Duration::from_secs(1);
/// assert_eq!(duration.as_secs(), 1);
/// assert!(matches!(
///     xsd::parse("PT1S", xsd::DURATION),
///     Err(xsd::ParseError::UnsupportedDatatype(datatype)) if datatype == xsd::DURATION
/// ));
/// ```
///
/// See: <https://www.w3.org/TR/xmlschema-2/#duration>
#[cfg(not(feature = "jiff"))]
pub use core::time::Duration;
