// This is free and unencumbered software released into the public domain.

use crate::{
    DecimalValue, ParseBooleanError, ParseDecimalError, ParseDoubleError, ParseError,
    ParseFloatError, Type, Value,
    primitives::{Boolean, Byte, Decimal, Double, Float, Int, Integer, Long, Short},
};
use core::convert::Infallible;

#[cfg(feature = "jiff")]
use crate::{
    ParseDateTimeError, ParseDurationError,
    primitives::{Date, DateTime, Duration, Time},
};

pub fn parse(input: impl AsRef<str>, datatype: impl Into<Type>) -> Result<Value, ParseError> {
    use crate::{DecimalType as D, PrimitiveType as P, Type::*};
    match datatype.into() {
        Decimal(D::Decimal) => parse_decimal(input),
        Decimal(D::Integer) => parse_integer(input),
        Decimal(D::Long) => parse_long(input),
        Decimal(D::Int) => parse_int(input),
        Decimal(D::Short) => parse_short(input),
        Decimal(D::Byte) => parse_byte(input),
        Primitive(P::String) => parse_string(input).map_err(|_| ParseError),
        Primitive(P::Boolean) => parse_boolean(input),
        Primitive(P::Decimal) => parse_decimal(input),
        Primitive(P::Float) => parse_float(input),
        Primitive(P::Double) => parse_double(input),
        #[cfg(feature = "jiff")]
        Primitive(P::Duration) => parse_duration(input),
        #[cfg(feature = "jiff")]
        Primitive(P::DateTime) => parse_datetime(input),
        #[cfg(feature = "jiff")]
        Primitive(P::Time) => parse_time(input),
        #[cfg(feature = "jiff")]
        Primitive(P::Date) => parse_date(input),
        Primitive(_) => todo!(),      // TODO
        Other(_) => unimplemented!(), // TODO
    }
}

pub fn parse_decimal(input: impl AsRef<str>) -> Result<Value, ParseDecimalError> {
    input
        .as_ref()
        .parse::<Decimal>()
        .map(Value::from)
        .map_err(|_| ParseError)
}

pub fn parse_integer(input: impl AsRef<str>) -> Result<Value, ParseDecimalError> {
    input
        .as_ref()
        .parse::<Integer>()
        .map(DecimalValue::from)
        .map(Value::from)
        .map_err(|_| ParseError)
}

pub fn parse_long(input: impl AsRef<str>) -> Result<Value, ParseDecimalError> {
    input
        .as_ref()
        .parse::<Long>()
        .map(DecimalValue::from)
        .map(Value::from)
        .map_err(|_| ParseError)
}

pub fn parse_int(input: impl AsRef<str>) -> Result<Value, ParseDecimalError> {
    input
        .as_ref()
        .parse::<Int>()
        .map(DecimalValue::from)
        .map(Value::from)
        .map_err(|_| ParseError)
}

pub fn parse_short(input: impl AsRef<str>) -> Result<Value, ParseDecimalError> {
    input
        .as_ref()
        .parse::<Short>()
        .map(DecimalValue::from)
        .map(Value::from)
        .map_err(|_| ParseError)
}

pub fn parse_byte(input: impl AsRef<str>) -> Result<Value, ParseDecimalError> {
    input
        .as_ref()
        .parse::<Byte>()
        .map(DecimalValue::from)
        .map(Value::from)
        .map_err(|_| ParseError)
}

#[cfg(feature = "alloc")]
pub fn parse_string(input: impl AsRef<str>) -> Result<Value, Infallible> {
    input
        .as_ref()
        .parse::<crate::primitives::String>()
        .map(Value::from)
}

#[cfg(not(feature = "alloc"))]
pub fn parse_string(_input: impl AsRef<str>) -> Result<Value, Infallible> {
    unimplemented!() // TODO
}

pub fn parse_boolean(input: impl AsRef<str>) -> Result<Value, ParseBooleanError> {
    input
        .as_ref()
        .parse::<Boolean>()
        .map(Value::from)
        .map_err(|_| ParseError)
}

pub fn parse_float(input: impl AsRef<str>) -> Result<Value, ParseFloatError> {
    input
        .as_ref()
        .parse::<Float>()
        .map(Value::from)
        .map_err(|_| ParseError)
}

pub fn parse_double(input: impl AsRef<str>) -> Result<Value, ParseDoubleError> {
    input
        .as_ref()
        .parse::<Double>()
        .map(Value::from)
        .map_err(|_| ParseError)
}

#[cfg(feature = "jiff")]
pub fn parse_duration(input: impl AsRef<str>) -> Result<Value, ParseDurationError> {
    input
        .as_ref()
        .parse::<Duration>()
        .map(Value::from)
        .map_err(|_| ParseError)
}

#[cfg(feature = "jiff")]
pub fn parse_datetime(input: impl AsRef<str>) -> Result<Value, ParseDateTimeError> {
    input
        .as_ref()
        .parse::<DateTime>()
        .map(Value::from)
        .map_err(|_| ParseError)
}

#[cfg(feature = "jiff")]
pub fn parse_time(input: impl AsRef<str>) -> Result<Value, ParseDateTimeError> {
    input
        .as_ref()
        .parse::<Time>()
        .map(Value::from)
        .map_err(|_| ParseError)
}

#[cfg(feature = "jiff")]
pub fn parse_date(input: impl AsRef<str>) -> Result<Value, ParseDateTimeError> {
    input
        .as_ref()
        .parse::<Date>()
        .map(Value::from)
        .map_err(|_| ParseError)
}
