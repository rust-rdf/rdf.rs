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
    ParseDateTimeError, ParseDurationError, ParseTemporalError,
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
#[cfg_attr(
    feature = "jiff",
    doc = "Temporal failures return [`ParseError::InvalidTemporal`], retaining the requested datatype and original Jiff error."
)]
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
        Primitive(P::Duration) => {
            parse_duration(input).map_err(|source| ParseError::InvalidTemporal {
                datatype: P::Duration,
                source: ParseTemporalError(source),
            })
        },
        #[cfg(feature = "jiff")]
        Primitive(P::DateTime) => {
            parse_datetime(input).map_err(|source| ParseError::InvalidTemporal {
                datatype: P::DateTime,
                source: ParseTemporalError(source),
            })
        },
        #[cfg(feature = "jiff")]
        Primitive(P::Time) => parse_time(input).map_err(|source| ParseError::InvalidTemporal {
            datatype: P::Time,
            source: ParseTemporalError(source),
        }),
        #[cfg(feature = "jiff")]
        Primitive(P::Date) => parse_date(input).map_err(|source| ParseError::InvalidTemporal {
            datatype: P::Date,
            source: ParseTemporalError(source),
        }),
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
///
/// Requires `jiff` (enabled by `datetime`). The date and time components must
/// be separated by uppercase ASCII `T`; lowercase `t` and space are rejected.
/// The time must begin with two-digit hours, minutes, and seconds separated by
/// colons (`hh:mm:ss`); omitted clock fields are not filled in with zero.
/// Fractional seconds use a period separator; a comma is rejected.
/// Numeric timezone offsets must be between `-14:00` and `+14:00`, inclusive.
/// The returned civil value currently does not retain accepted timezone offsets.
///
/// # Errors
///
/// Returns an error when the input cannot be parsed by the underlying
/// civil-dateTime parser, does not use the required uppercase `T` separator,
/// omits the required `hh:mm:ss` clock fields, uses a comma to separate fractional
/// seconds, or has a numeric timezone offset outside the XSD range.
///
/// ```
/// let value = xsd::parse_datetime("2026-12-31T12:34:56").unwrap();
/// assert_eq!(value.to_string(), "2026-12-31T12:34:56");
/// assert!(xsd::parse_datetime("2026-12-31 12:34:56").is_err());
/// assert!(xsd::parse_datetime("2026-12-31t12:34:56").is_err());
/// assert!(xsd::parse_datetime("2026-12-31T12:34").is_err());
/// assert!(xsd::parse_datetime("2026-12-31T12:34:56,125").is_err());
/// assert!(xsd::parse_datetime("2026-12-31T12:34:56+14:00").is_ok());
/// assert!(xsd::parse_datetime("2026-12-31T12:34:56+14:01").is_err());
/// ```
#[cfg(feature = "jiff")]
pub fn parse_datetime(input: impl AsRef<str>) -> Result<Value, ParseDateTimeError> {
    let input = input.as_ref();
    let datetime = input.parse::<DateTime>()?;
    // Jiff accepts T, t, or space here; XSD only permits uppercase T.
    let separator = input
        .bytes()
        .position(|byte| matches!(byte, b'T' | b't' | b' '));
    let Some(separator) = separator.filter(|&index| input.as_bytes()[index] == b'T') else {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:dateTime literals require an uppercase T separator"
        )));
    };
    // Jiff validates the digits and ranges; require all three clock fields.
    if !matches!(
        &input.as_bytes()[separator + 1..],
        [_, _, b':', _, _, b':', _, _, ..]
    ) {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:dateTime literals require hours, minutes, and seconds (hh:mm:ss)"
        )));
    }
    if input.as_bytes().get(separator + 9) == Some(&b',') {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:dateTime fractional seconds require a period separator"
        )));
    }
    if jiff::fmt::temporal::DateTimeParser::new()
        .parse_pieces(input)?
        .to_numeric_offset()
        .is_some_and(|offset| offset.seconds().unsigned_abs() > 14 * 60 * 60)
    {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:dateTime timezone offsets must be between -14:00 and +14:00"
        )));
    }
    Ok(Value::from(datetime))
}

/// Parses an input string containing an `xsd:time` literal.
///
/// Requires `jiff` (enabled by `datetime`). Inputs must begin with two-digit
/// hours, minutes, and seconds separated by colons (`hh:mm:ss`). Seconds may
/// include a fractional part; omitted clock fields are not filled in with zero.
/// Fractional seconds use a period separator; a comma is rejected.
///
/// # Errors
///
/// Returns an error when the input cannot be parsed by the underlying civil-time
/// parser, does not begin with the required `hh:mm:ss` clock fields, or uses a
/// comma to separate fractional seconds.
///
/// ```
/// assert!(xsd::parse_time("12:34").is_err());
/// assert!(xsd::parse_time("12:34:56,125").is_err());
/// assert_eq!(xsd::parse_time("12:34:00").unwrap().to_string(), "12:34:00");
/// assert_eq!(xsd::parse_time("12:34:56.125").unwrap().to_string(), "12:34:56.125");
/// ```
#[cfg(feature = "jiff")]
pub fn parse_time(input: impl AsRef<str>) -> Result<Value, ParseDateTimeError> {
    let input = input.as_ref();
    let time = input.parse::<Time>()?;
    // Jiff validates the digits and ranges; require all three colon-separated fields.
    if !matches!(input.as_bytes(), [_, _, b':', _, _, b':', _, _, ..]) {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:time literals require hours, minutes, and seconds (hh:mm:ss)"
        )));
    }
    if input.as_bytes().get(8) == Some(&b',') {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:time fractional seconds require a period separator"
        )));
    }
    Ok(Value::from(time))
}

/// Parses an input string containing an `xsd:date` literal.
///
/// Requires `jiff` (enabled by `datetime`). Inputs containing a time component
/// are rejected instead of being truncated to their date component.
///
/// # Errors
///
/// Returns an error when the input contains a time component or cannot be
/// parsed by the underlying civil-date parser.
///
/// ```
/// assert!(xsd::parse_date("2026-12-31T12:34:56").is_err());
/// assert_eq!(xsd::parse_date("2024-02-29").unwrap().to_string(), "2024-02-29");
/// ```
#[cfg(feature = "jiff")]
pub fn parse_date(input: impl AsRef<str>) -> Result<Value, ParseDateTimeError> {
    let input = input.as_ref();
    if jiff::fmt::temporal::DateTimeParser::new()
        .parse_pieces(input)?
        .time()
        .is_some()
    {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:date literals must not contain a time component"
        )));
    }
    // Preserve the civil-date parser's other checks, including offset handling.
    input.parse::<Date>().map(Value::from)
}
