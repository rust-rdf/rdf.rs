// This is free and unencumbered software released into the public domain.

use crate::{DecimalType, PrimitiveType, Type};

/// An error encountered when parsing an `xsd:boolean` literal.
///
/// [`crate::parse_boolean`] reports [`ParseError::InvalidBoolean`] through this alias.
pub type ParseBooleanError = ParseError;

/// An error encountered when parsing an `xsd:double` literal.
pub type ParseDoubleError = core::num::ParseFloatError;

/// An error encountered when parsing an `xsd:float` literal.
pub type ParseFloatError = core::num::ParseFloatError;

/// An error encountered when parsing an integer-family literal.
///
/// Its [`kind()`](core::num::ParseIntError::kind) distinguishes lexical failures
/// from values outside the parser's supported range.
pub type ParseIntegerError = core::num::ParseIntError;

/// An error encountered when parsing an `xsd:dateTime` literal.
#[cfg(feature = "jiff")]
pub type ParseDateTimeError = jiff::Error;

/// An error encountered when parsing an `xsd:duration` literal.
#[cfg(feature = "jiff")]
pub type ParseDurationError = jiff::Error;

/// A temporal parsing error exposed through [`core::error::Error`].
///
/// Requires `jiff`. Retains the native error in its tuple field and provides the
/// standard error trait across feature configurations, including `no_std`.
/// Wrapping adds no allocation.
#[cfg(feature = "jiff")]
#[derive(Debug)]
pub struct ParseTemporalError(
    /// The underlying temporal parsing error, represented as a Jiff error.
    pub jiff::Error,
);

#[cfg(feature = "jiff")]
impl core::fmt::Display for ParseTemporalError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.0, f)
    }
}

#[cfg(feature = "jiff")]
impl core::error::Error for ParseTemporalError {}

/// An error encountered when parsing an XSD literal.
///
/// Distinguishes an unsupported datatype from a failure in an available parser.
/// This error is available without `alloc` or `std`.
#[derive(Debug)]
#[non_exhaustive]
pub enum ParseError {
    /// The selected parser could not parse or represent the literal.
    ///
    /// The underlying parser's cause is not retained.
    InvalidLiteral,

    /// A literal could not be parsed as `xsd:boolean`.
    ///
    /// The accepted spellings are `true`, `false`, `1`, and `0`. The boolean
    /// backend supplies no underlying cause, so [`core::error::Error::source`]
    /// returns `None`.
    ///
    /// ```
    /// assert!(matches!(
    ///     xsd::parse_boolean("yes"),
    ///     Err(xsd::ParseError::InvalidBoolean)
    /// ));
    /// ```
    InvalidBoolean,

    /// An `xsd:integer`, `long`, `int`, `short`, or `byte` parser failed.
    ///
    /// The original error distinguishes lexical failures from representation
    /// limits and is also exposed through [`core::error::Error::source`].
    ///
    /// ```
    /// let error = xsd::parse("128", xsd::BYTE).unwrap_err();
    /// let xsd::ParseError::InvalidInteger { datatype, source } = error else {
    ///     panic!("expected an integer parse error");
    /// };
    /// assert_eq!(datatype, xsd::DecimalType::Byte);
    /// assert_eq!(source.kind(), &core::num::IntErrorKind::PosOverflow);
    /// ```
    InvalidInteger {
        /// The requested integer-family datatype.
        datatype: DecimalType,
        /// The underlying parser error, including its [`core::num::IntErrorKind`].
        source: ParseIntegerError,
    },

    /// An `xsd:decimal` parser failed.
    ///
    /// The backend error is retained, including any range or precision failure
    /// it reports, and is also exposed through [`core::error::Error::source`].
    ///
    /// ```
    /// let error = xsd::parse("not-a-decimal", xsd::DECIMAL).unwrap_err();
    /// assert!(matches!(
    ///     error,
    ///     xsd::ParseError::InvalidDecimal { datatype, .. } if datatype == xsd::DECIMAL
    /// ));
    /// ```
    InvalidDecimal {
        /// The requested datatype, preserving its [`Type`] representation.
        datatype: Type,
        /// The original decimal backend error.
        source: ParseDecimalError,
    },

    /// An `xsd:float` or `xsd:double` parser failed.
    ///
    /// Retains the requested floating-point datatype and the original parser
    /// error, also exposed through [`core::error::Error::source`].
    ///
    /// ```
    /// let error = xsd::parse("1e+", xsd::DOUBLE).unwrap_err();
    /// assert!(matches!(
    ///     error,
    ///     xsd::ParseError::InvalidFloat { datatype: xsd::PrimitiveType::Double, .. }
    /// ));
    /// ```
    InvalidFloat {
        /// The requested datatype: [`PrimitiveType::Float`] or [`PrimitiveType::Double`].
        datatype: PrimitiveType,
        /// The original parser error; [`ParseDoubleError`] uses the same underlying type.
        source: ParseFloatError,
    },

    /// An `xsd:date`, `dateTime`, `time`, or `duration` parser failed.
    ///
    /// Requires `jiff`. Retains the requested datatype and original Jiff error.
    /// [`core::error::Error::source`] exposes the [`ParseTemporalError`] wrapper;
    /// its tuple field provides access to the native Jiff error.
    ///
    /// ```
    /// let error = xsd::parse("2026-02-29", xsd::DATE).unwrap_err();
    /// assert!(matches!(
    ///     error,
    ///     xsd::ParseError::InvalidTemporal { datatype: xsd::PrimitiveType::Date, .. }
    /// ));
    /// ```
    #[cfg(feature = "jiff")]
    InvalidTemporal {
        /// The requested date, dateTime, time, or duration datatype.
        datatype: PrimitiveType,
        /// The original parser error adapted for standard error chaining.
        source: ParseTemporalError,
    },

    /// No parser is available for this datatype in the enabled feature set.
    ///
    /// Includes unknown datatypes and datatypes whose required feature is disabled.
    UnsupportedDatatype(Type),
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::InvalidLiteral => f.write_str("invalid XSD literal"),
            Self::InvalidBoolean => {
                f.write_str("invalid xsd:boolean literal: expected true, false, 1, or 0")
            },
            Self::InvalidInteger { datatype, source } => {
                write!(f, "invalid {} literal: {source}", datatype.curie())
            },
            Self::InvalidDecimal { datatype, source } => {
                write!(f, "invalid {} literal: {source}", datatype.curie())
            },
            Self::InvalidFloat { datatype, source } => {
                write!(f, "invalid {} literal: {source}", datatype.curie())
            },
            #[cfg(feature = "jiff")]
            Self::InvalidTemporal { datatype, source } => {
                write!(f, "invalid {} literal: {source}", datatype.curie())
            },
            Self::UnsupportedDatatype(datatype) => {
                write!(f, "unsupported datatype: {}", datatype.curie())
            },
        }
    }
}

impl core::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::InvalidInteger { source, .. } => Some(source),
            Self::InvalidDecimal { source, .. } => Some(source),
            Self::InvalidFloat { source, .. } => Some(source),
            #[cfg(feature = "jiff")]
            Self::InvalidTemporal { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// An error reported by the backend when parsing an `xsd:decimal` literal.
///
/// Preserves the backend's diagnostics for malformed input and any reported
/// representation limits.
pub type ParseDecimalError = valuand::DecimalError;
