// This is free and unencumbered software released into the public domain.

extern crate alloc;

use crate::PrimitiveType;
use alloc::{string::String, vec::Vec};
use float_ord::FloatOrd;

/// Values based on built-in primitive datatypes.
///
/// See: https://www.w3.org/TR/xmlschema-2/#built-in-datatypes
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PrimitiveValue {
    /// See: https://www.w3.org/TR/xmlschema-2/#string
    String(String),

    /// See: https://www.w3.org/TR/xmlschema-2/#boolean
    Boolean(bool),

    /// See: https://www.w3.org/TR/xmlschema-2/#decimal
    Decimal(rust_decimal::Decimal),

    /// See: https://www.w3.org/TR/xmlschema-2/#float
    Float(FloatOrd<f32>),

    /// See: https://www.w3.org/TR/xmlschema-2/#double
    Double(FloatOrd<f64>),

    /// See: https://www.w3.org/TR/xmlschema-2/#duration
    Duration(jiff::SignedDuration),

    /// See: https://www.w3.org/TR/xmlschema-2/#dateTime
    DateTime(jiff::civil::DateTime),

    /// See: https://www.w3.org/TR/xmlschema-2/#time
    Time(jiff::civil::Time),

    /// See: https://www.w3.org/TR/xmlschema-2/#date
    Date(jiff::civil::Date),

    /// See: https://www.w3.org/TR/xmlschema-2/#gYearMonth
    GYearMonth(i32, u8),

    /// See: https://www.w3.org/TR/xmlschema-2/#gYear
    GYear(i32),

    /// See: https://www.w3.org/TR/xmlschema-2/#gMonthDay
    GMonthDay(u8, u8),

    /// See: https://www.w3.org/TR/xmlschema-2/#gDay
    GDay(u8),

    /// See: https://www.w3.org/TR/xmlschema-2/#gMonth
    GMonth(u8),

    /// See: https://www.w3.org/TR/xmlschema-2/#hexBinary
    HexBinary(Vec<u8>),

    /// See: https://www.w3.org/TR/xmlschema-2/#base64Binary
    Base64Binary(Vec<u8>),

    /// See: https://www.w3.org/TR/xmlschema-2/#anyURI
    AnyURI(String),

    /// See: https://www.w3.org/TR/xmlschema-2/#QName
    QName(String, String),
}

impl PrimitiveValue {
    pub fn r#type(&self) -> PrimitiveType {
        use PrimitiveValue::*;
        match self {
            String(_) => PrimitiveType::String,
            Boolean(_) => PrimitiveType::Boolean,
            Decimal(_) => PrimitiveType::Decimal,
            Float(_) => PrimitiveType::Float,
            Double(_) => PrimitiveType::Double,
            Duration(_) => PrimitiveType::Duration,
            DateTime(_) => PrimitiveType::DateTime,
            Time(_) => PrimitiveType::Time,
            Date(_) => PrimitiveType::Date,
            GYearMonth(_, _) => PrimitiveType::GYearMonth,
            GYear(_) => PrimitiveType::GYear,
            GMonthDay(_, _) => PrimitiveType::GMonthDay,
            GDay(_) => PrimitiveType::GDay,
            GMonth(_) => PrimitiveType::GMonth,
            HexBinary(_) => PrimitiveType::HexBinary,
            Base64Binary(_) => PrimitiveType::Base64Binary,
            AnyURI(_) => PrimitiveType::AnyURI,
            QName(_, _) => PrimitiveType::QName,
        }
    }
}
