// This is free and unencumbered software released into the public domain.

use crate::{DecimalType, Type};

/// An error encountered when parsing an `xsd:boolean` literal.
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

    /// No parser is available for this datatype in the enabled feature set.
    ///
    /// Includes unknown datatypes and datatypes whose required feature is disabled.
    UnsupportedDatatype(Type),
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::InvalidLiteral => f.write_str("invalid XSD literal"),
            Self::InvalidInteger { datatype, source } => {
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
            _ => None,
        }
    }
}

pub type ParseDecimalError = valuand::DecimalError;
