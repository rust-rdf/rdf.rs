// This is free and unencumbered software released into the public domain.

use crate::Type;

/// An error encountered when parsing an `xsd:boolean` literal.
pub type ParseBooleanError = ParseError;

/// An error encountered when parsing an `xsd:double` literal.
pub type ParseDoubleError = core::num::ParseFloatError;

/// An error encountered when parsing an `xsd:float` literal.
pub type ParseFloatError = core::num::ParseFloatError;

/// An error encountered when parsing an `xsd:integer` literal.
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

    /// No parser is available for this datatype in the enabled feature set.
    ///
    /// Includes unknown datatypes and datatypes whose required feature is disabled.
    UnsupportedDatatype(Type),
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::InvalidLiteral => f.write_str("invalid XSD literal"),
            Self::UnsupportedDatatype(datatype) => {
                write!(f, "unsupported datatype: {}", datatype.curie())
            },
        }
    }
}

impl core::error::Error for ParseError {}

pub type ParseDecimalError = valuand::DecimalError;
