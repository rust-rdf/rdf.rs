// This is free and unencumbered software released into the public domain.

use crate::{
    DecimalValue, ParseBooleanError, ParseDecimalError, ParseDoubleError, ParseError,
    ParseFloatError, ParseIntegerError, Type, Value,
    derived::{Byte, Int, Integer, Long, Short},
    primitive::{Boolean, Decimal, Double, Float},
};
use core::convert::Infallible;

#[cfg(feature = "jiff")]
use crate::{
    ParseDateTimeError, ParseDurationError,
    primitive::{Date, DateTime, Duration, Time},
};

/// Parses an input string containing an XSD literal.
pub fn parse(input: impl AsRef<str>, datatype: impl Into<Type>) -> Result<Value, ParseError> {
    use crate::{DecimalType as D, PrimitiveType as P, Type::*};
    match datatype.into() {
        Decimal(D::Decimal) => parse_decimal(input).map_err(|_| ParseError),
        Decimal(D::Integer) => parse_integer(input).map_err(|_| ParseError),
        Decimal(D::Long) => parse_long(input).map_err(|_| ParseError),
        Decimal(D::Int) => parse_int(input).map_err(|_| ParseError),
        Decimal(D::Short) => parse_short(input).map_err(|_| ParseError),
        Decimal(D::Byte) => parse_byte(input).map_err(|_| ParseError),
        Primitive(P::String) => parse_string(input).map_err(|_| ParseError),
        Primitive(P::Boolean) => parse_boolean(input).map_err(|_| ParseError),
        Primitive(P::Decimal) => parse_decimal(input).map_err(|_| ParseError),
        Primitive(P::Float) => parse_float(input).map_err(|_| ParseError),
        Primitive(P::Double) => parse_double(input).map_err(|_| ParseError),
        #[cfg(feature = "jiff")]
        Primitive(P::Duration) => parse_duration(input).map_err(|_| ParseError),
        #[cfg(feature = "jiff")]
        Primitive(P::DateTime) => parse_datetime(input).map_err(|_| ParseError),
        #[cfg(feature = "jiff")]
        Primitive(P::Time) => parse_time(input).map_err(|_| ParseError),
        #[cfg(feature = "jiff")]
        Primitive(P::Date) => parse_date(input).map_err(|_| ParseError),
        Primitive(_) => todo!(),      // TODO
        Other(_) => unimplemented!(), // TODO
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
#[cfg(feature = "alloc")]
pub fn parse_string(input: impl AsRef<str>) -> Result<Value, Infallible> {
    use crate::primitive::String;
    input.as_ref().parse::<String>().map(Value::from)
}

#[cfg(not(feature = "alloc"))]
/// Parses an input string containing an `xsd:string` literal.
pub fn parse_string(_input: impl AsRef<str>) -> Result<Value, Infallible> {
    unimplemented!() // TODO
}

/// Parses an input string containing an `xsd:boolean` literal.
pub fn parse_boolean(input: impl AsRef<str>) -> Result<Value, ParseBooleanError> {
    input
        .as_ref()
        .parse::<Boolean>()
        .map(Value::from)
        .map_err(|_| ParseError)
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
