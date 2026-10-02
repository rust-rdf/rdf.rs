// This is free and unencumbered software released into the public domain.

use crate::{
    DecimalValue, ParseBooleanError, ParseDecimalError, ParseDoubleError, ParseError,
    ParseFloatError, ParseIntegerError, Type, Value,
    derived::{Byte, Int, Integer, Long, Short},
    primitive::{Boolean, Decimal, Double, Float},
};

#[cfg(feature = "alloc")]
use core::convert::Infallible;

#[cfg(feature = "jiff")]
use crate::{
    ParseDateTimeError, ParseDurationError,
    primitive::{Date, DateTime, Duration, Time},
};

/// Parses an input string containing an XSD literal.
///
/// # Errors
///
/// Returns [`ParseError::UnsupportedDatatype`] when no parser is available for
/// the datatype, including unknown datatypes. String parsing requires `alloc`;
/// date, time, dateTime, and duration parsing require `jiff` (enabled by `datetime`).
/// Disabled capabilities return the same error. Integer-family parser failures
/// return [`ParseError::InvalidInteger`]; decimal failures return
/// [`ParseError::InvalidDecimal`]; float/double failures return
/// [`ParseError::InvalidFloat`]. These numeric errors retain the requested datatype
/// and underlying cause. Boolean failures return [`ParseError::InvalidBoolean`].
/// Other failures from available parsers return [`ParseError::InvalidLiteral`].
///
/// ```
/// let error = xsd::parse("AQI=", xsd::BASE64_BINARY).unwrap_err();
/// assert!(matches!(
///     error,
///     xsd::ParseError::UnsupportedDatatype(datatype) if datatype == xsd::BASE64_BINARY
/// ));
/// ```
pub fn parse(input: impl AsRef<str>, datatype: impl Into<Type>) -> Result<Value, ParseError> {
    use crate::{DecimalType as D, PrimitiveType as P, Type::*};
    match datatype.into() {
        datatype @ (Decimal(D::Decimal) | Primitive(P::Decimal)) => {
            parse_decimal(input).map_err(|source| ParseError::InvalidDecimal { datatype, source })
        },
        Decimal(D::Integer) => parse_integer(input).map_err(|source| ParseError::InvalidInteger {
            datatype: D::Integer,
            source,
        }),
        Decimal(D::Long) => parse_long(input).map_err(|source| ParseError::InvalidInteger {
            datatype: D::Long,
            source,
        }),
        Decimal(D::Int) => parse_int(input).map_err(|source| ParseError::InvalidInteger {
            datatype: D::Int,
            source,
        }),
        Decimal(D::Short) => parse_short(input).map_err(|source| ParseError::InvalidInteger {
            datatype: D::Short,
            source,
        }),
        Decimal(D::Byte) => parse_byte(input).map_err(|source| ParseError::InvalidInteger {
            datatype: D::Byte,
            source,
        }),
        #[cfg(feature = "alloc")]
        Primitive(P::String) => parse_string(input).map_err(|_| ParseError::InvalidLiteral),
        Primitive(P::Boolean) => parse_boolean(input),
        Primitive(P::Float) => parse_float(input).map_err(|source| ParseError::InvalidFloat {
            datatype: P::Float,
            source,
        }),
        Primitive(P::Double) => parse_double(input).map_err(|source| ParseError::InvalidFloat {
            datatype: P::Double,
            source,
        }),
        #[cfg(feature = "jiff")]
        Primitive(P::Duration) => parse_duration(input).map_err(|_| ParseError::InvalidLiteral),
        #[cfg(feature = "jiff")]
        Primitive(P::DateTime) => parse_datetime(input).map_err(|_| ParseError::InvalidLiteral),
        #[cfg(feature = "jiff")]
        Primitive(P::Time) => parse_time(input).map_err(|_| ParseError::InvalidLiteral),
        #[cfg(feature = "jiff")]
        Primitive(P::Date) => parse_date(input).map_err(|_| ParseError::InvalidLiteral),
        datatype => Err(ParseError::UnsupportedDatatype(datatype)),
    }
}

/// Parses an input string containing an `xsd:decimal` literal.
pub fn parse_decimal(input: impl AsRef<str>) -> Result<Value, ParseDecimalError> {
    input.as_ref().parse::<Decimal>().map(Value::from)
}

/// Parses an input string containing an `xsd:integer` literal.
pub fn parse_integer(input: impl AsRef<str>) -> Result<Value, ParseIntegerError> {
    input
        .as_ref()
        .parse::<Integer>()
        .map(DecimalValue::from)
        .map(Value::from)
}

/// Parses an input string containing an `xsd:long` literal.
pub fn parse_long(input: impl AsRef<str>) -> Result<Value, ParseIntegerError> {
    input
        .as_ref()
        .parse::<Long>()
        .map(DecimalValue::from)
        .map(Value::from)
}

/// Parses an input string containing an `xsd:int` literal.
pub fn parse_int(input: impl AsRef<str>) -> Result<Value, ParseIntegerError> {
    input
        .as_ref()
        .parse::<Int>()
        .map(DecimalValue::from)
        .map(Value::from)
}

/// Parses an input string containing an `xsd:short` literal.
pub fn parse_short(input: impl AsRef<str>) -> Result<Value, ParseIntegerError> {
    input
        .as_ref()
        .parse::<Short>()
        .map(DecimalValue::from)
        .map(Value::from)
}

/// Parses an input string containing an `xsd:byte` literal.
pub fn parse_byte(input: impl AsRef<str>) -> Result<Value, ParseIntegerError> {
    input
        .as_ref()
        .parse::<Byte>()
        .map(DecimalValue::from)
        .map(Value::from)
}

/// Parses an input string containing an `xsd:string` literal.
///
/// With `alloc` enabled, copies the input verbatim into an owned string and
/// returns an infallible result. Without `alloc`, the error type is [`ParseError`]
/// and parsing returns [`ParseError::UnsupportedDatatype`] for [`crate::STRING`].
#[cfg(feature = "alloc")]
pub fn parse_string(input: impl AsRef<str>) -> Result<Value, Infallible> {
    use crate::primitive::String;
    input.as_ref().parse::<String>().map(Value::from)
}

/// Parses an input string containing an `xsd:string` literal.
///
/// String parsing requires `alloc`. With that feature enabled, this function
/// copies the input verbatim into an owned string and its error type is
/// [`core::convert::Infallible`].
///
/// # Errors
///
/// Without `alloc`, always returns [`ParseError::UnsupportedDatatype`] for
/// [`crate::STRING`], including for empty or static input.
///
/// ```
/// assert!(matches!(
///     xsd::parse_string("hello"),
///     Err(xsd::ParseError::UnsupportedDatatype(datatype)) if datatype == xsd::STRING
/// ));
/// ```
#[cfg(not(feature = "alloc"))]
pub fn parse_string(_input: impl AsRef<str>) -> Result<Value, ParseError> {
    Err(ParseError::UnsupportedDatatype(crate::STRING))
}

/// Parses an input string containing an `xsd:boolean` literal.
///
/// Accepts the exact lexical forms `true`, `false`, `1`, and `0`.
///
/// # Errors
///
/// Returns [`ParseError::InvalidBoolean`] when the literal cannot be parsed.
pub fn parse_boolean(input: impl AsRef<str>) -> Result<Value, ParseBooleanError> {
    input
        .as_ref()
        .parse::<Boolean>()
        .map(Value::from)
        .map_err(|_| ParseError::InvalidBoolean)
}

/// Parses an input string containing an `xsd:float` literal.
pub fn parse_float(input: impl AsRef<str>) -> Result<Value, ParseFloatError> {
    input.as_ref().parse::<Float>().map(Value::from)
}

/// Parses an input string containing an `xsd:double` literal.
pub fn parse_double(input: impl AsRef<str>) -> Result<Value, ParseDoubleError> {
    input.as_ref().parse::<Double>().map(Value::from)
}

/// Parses an input string containing an `xsd:duration` literal.
#[cfg(feature = "jiff")]
pub fn parse_duration(input: impl AsRef<str>) -> Result<Value, ParseDurationError> {
    input.as_ref().parse::<Duration>().map(Value::from)
}

/// Parses an input string containing an `xsd:dateTime` literal.
#[cfg(feature = "jiff")]
pub fn parse_datetime(input: impl AsRef<str>) -> Result<Value, ParseDateTimeError> {
    input.as_ref().parse::<DateTime>().map(Value::from)
}

/// Parses an input string containing an `xsd:time` literal.
#[cfg(feature = "jiff")]
pub fn parse_time(input: impl AsRef<str>) -> Result<Value, ParseDateTimeError> {
    input.as_ref().parse::<Time>().map(Value::from)
}

/// Parses an input string containing an `xsd:date` literal.
#[cfg(feature = "jiff")]
pub fn parse_date(input: impl AsRef<str>) -> Result<Value, ParseDateTimeError> {
    input.as_ref().parse::<Date>().map(Value::from)
}
