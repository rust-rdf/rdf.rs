// This is free and unencumbered software released into the public domain.

use crate::primitive::{Boolean, GDay, GMonth, GMonthDay, GYear, GYearMonth};
use crate::{
    PrimitiveType,
    primitive::{Decimal, Double, Float},
};

#[cfg(feature = "jiff")]
use crate::primitive::{Date, DateTime, Duration, Time};

#[cfg(feature = "alloc")]
use strum_macros::Display;

#[cfg(feature = "alloc")]
use ::alloc::{borrow::Cow, format, string::String, vec::Vec};

/// Value representation for XSD primitive datatypes.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#built-in-datatypes>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "alloc", derive(Display))]
// #[cfg_attr(
//     feature = "borsh",
//     derive(borsh::BorshSerialize, borsh::BorshDeserialize) // FIXME: SignedDuration
// )]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrimitiveValue {
    /// See: <https://www.w3.org/TR/xmlschema-2/#string>
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    String(String),
    #[cfg(not(feature = "alloc"))]
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    String(&'static str),

    /// See: <https://www.w3.org/TR/xmlschema-2/#boolean>
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Boolean(Boolean),

    /// See: <https://www.w3.org/TR/xmlschema-2/#decimal>
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Decimal(Decimal),

    /// See: <https://www.w3.org/TR/xmlschema-2/#float>
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Float(Float),

    /// See: <https://www.w3.org/TR/xmlschema-2/#double>
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Double(Double),

    /// See: <https://www.w3.org/TR/xmlschema-2/#duration>
    #[cfg(feature = "jiff")]
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Duration(Duration),

    /// See: <https://www.w3.org/TR/xmlschema-2/#dateTime>
    #[cfg(feature = "jiff")]
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    DateTime(DateTime),

    /// See: <https://www.w3.org/TR/xmlschema-2/#time>
    #[cfg(feature = "jiff")]
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Time(Time),

    /// See: <https://www.w3.org/TR/xmlschema-2/#date>
    #[cfg(feature = "jiff")]
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Date(Date),

    /// See: <https://www.w3.org/TR/xmlschema-2/#gYearMonth>
    //#[cfg_attr(feature = "alloc", strum(to_string = "{0.0}-{0.1}"))]
    GYearMonth(GYearMonth),

    /// See: <https://www.w3.org/TR/xmlschema-2/#gYear>
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    GYear(GYear),

    /// See: <https://www.w3.org/TR/xmlschema-2/#gMonthDay>
    //#[cfg_attr(feature = "alloc", strum(to_string = "{0}-{1}"))]
    GMonthDay(GMonthDay),

    /// See: <https://www.w3.org/TR/xmlschema-2/#gDay>
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    GDay(GDay),

    /// See: <https://www.w3.org/TR/xmlschema-2/#gMonth>
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    GMonth(GMonth),

    /// See: <https://www.w3.org/TR/xmlschema-2/#hexBinary>
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "alloc", strum(to_string = "HexBinary"))]
    HexBinary(Vec<u8>),

    /// See: <https://www.w3.org/TR/xmlschema-2/#base64Binary>
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "alloc", strum(to_string = "Base64Binary"))]
    Base64Binary(Vec<u8>),

    /// See: <https://www.w3.org/TR/xmlschema-2/#anyURI>
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    AnyUri(String),

    /// See: <https://www.w3.org/TR/xmlschema-2/#QName>
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}:{1}"))]
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
            #[cfg(feature = "jiff")]
            Duration(_) => PrimitiveType::Duration,
            #[cfg(feature = "jiff")]
            DateTime(_) => PrimitiveType::DateTime,
            #[cfg(feature = "jiff")]
            Time(_) => PrimitiveType::Time,
            #[cfg(feature = "jiff")]
            Date(_) => PrimitiveType::Date,
            GYearMonth(_) => PrimitiveType::GYearMonth,
            GYear(_) => PrimitiveType::GYear,
            GMonthDay(_) => PrimitiveType::GMonthDay,
            GDay(_) => PrimitiveType::GDay,
            GMonth(_) => PrimitiveType::GMonth,
            #[cfg(feature = "alloc")]
            HexBinary(_) => PrimitiveType::HexBinary,
            #[cfg(feature = "alloc")]
            Base64Binary(_) => PrimitiveType::Base64Binary,
            #[cfg(feature = "alloc")]
            AnyUri(_) => PrimitiveType::AnyUri,
            #[cfg(feature = "alloc")]
            QName(_, _) => PrimitiveType::QName,
        }
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Option<serde_json::Value> {
        Some(self.clone().into_json())
    }

    #[cfg(feature = "serde")]
    pub fn into_json(self) -> serde_json::Value {
        use PrimitiveValue::*;
        #[cfg(feature = "jiff")]
        use alloc::string::ToString;
        use serde_json::Value;
        #[allow(unused)]
        match self {
            String(s) => Value::String(s),
            Boolean(b) => b.into_json(),
            Decimal(d) => d.into_json().unwrap(),
            Float(f) => f.into_json(),
            Double(d) => d.into_json(),
            #[cfg(feature = "jiff")]
            Duration(d) => Value::String(d.to_string()),
            #[cfg(feature = "jiff")]
            DateTime(d) => Value::String(d.to_string()),
            #[cfg(feature = "jiff")]
            Time(t) => Value::String(t.to_string()),
            #[cfg(feature = "jiff")]
            Date(d) => Value::String(d.to_string()),
            GYearMonth((y, m)) => Value::String(format!("{}-{}", y, m)),
            GYear(y) => Value::String(y.to_string()),
            GMonthDay((m, d)) => Value::String(format!("{}-{}", m, d)),
            GDay(d) => Value::String(d.to_string()),
            GMonth(m) => Value::String(m.to_string()),
            #[cfg(feature = "alloc")]
            HexBinary(_b) => Value::String(todo!()), // FIXME
            #[cfg(feature = "alloc")]
            Base64Binary(_b) => Value::String(todo!()), // FIXME
            #[cfg(feature = "alloc")]
            AnyUri(u) => Value::String(u),
            #[cfg(feature = "alloc")]
            QName(n, s) => Value::String(format!("{}:{}", n, s)),
        }
    }

    #[cfg(feature = "bson")]
    pub fn to_bson(&self) -> Option<bson::Bson> {
        Some(self.clone().into_bson())
    }

    #[cfg(feature = "bson")]
    pub fn into_bson(self) -> bson::Bson {
        use PrimitiveValue::*;
        #[cfg(feature = "jiff")]
        use alloc::string::ToString;
        use bson::{Binary, Bson, spec::BinarySubtype};
        #[allow(unused)]
        match self {
            String(s) => Bson::String(s),
            Boolean(b) => b.into_bson(),
            Decimal(d) => d.into_bson().unwrap(),
            Float(f) => f.into_bson(),
            Double(d) => d.into_bson(),
            #[cfg(feature = "jiff")]
            Duration(d) => Bson::String(d.to_string()),
            #[cfg(feature = "jiff")]
            DateTime(d) => Bson::String(d.to_string()),
            #[cfg(feature = "jiff")]
            Time(t) => Bson::String(t.to_string()),
            #[cfg(feature = "jiff")]
            Date(d) => Bson::String(d.to_string()),
            GYearMonth((y, m)) => Bson::String(format!("{}-{}", y, m)),
            GYear(y) => Bson::String(y.to_string()),
            GMonthDay((m, d)) => Bson::String(format!("{}-{}", m, d)),
            GDay(d) => Bson::String(d.to_string()),
            GMonth(m) => Bson::String(m.to_string()),
            #[cfg(feature = "alloc")]
            HexBinary(b) => Bson::Binary(Binary {
                bytes: b,
                subtype: BinarySubtype::Generic,
            }),
            #[cfg(feature = "alloc")]
            Base64Binary(b) => Bson::Binary(Binary {
                bytes: b,
                subtype: BinarySubtype::Generic,
            }),
            #[cfg(feature = "alloc")]
            AnyUri(u) => Bson::String(u.to_string()),
            #[cfg(feature = "alloc")]
            QName(n, s) => Bson::String(format!("{}:{}", n, s)),
        }
    }
}

impl<T> From<&T> for PrimitiveValue
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<&'static str> for PrimitiveValue {
    fn from(input: &'static str) -> Self {
        Self::String(input.into())
    }
}

#[cfg(feature = "alloc")]
impl From<Cow<'_, str>> for PrimitiveValue {
    fn from(input: Cow<'_, str>) -> Self {
        Self::String(input.into())
    }
}

#[cfg(feature = "alloc")]
impl From<String> for PrimitiveValue {
    fn from(input: String) -> Self {
        Self::String(input)
    }
}

impl From<bool> for PrimitiveValue {
    fn from(input: bool) -> Self {
        Self::Boolean(input.into())
    }
}

impl From<Boolean> for PrimitiveValue {
    fn from(input: Boolean) -> Self {
        Self::Boolean(input)
    }
}

impl From<i8> for PrimitiveValue {
    fn from(input: i8) -> Self {
        Self::Decimal(input.into())
    }
}

impl From<i16> for PrimitiveValue {
    fn from(input: i16) -> Self {
        Self::Decimal(input.into())
    }
}

impl From<i32> for PrimitiveValue {
    fn from(input: i32) -> Self {
        Self::Decimal(input.into())
    }
}

impl From<i64> for PrimitiveValue {
    fn from(input: i64) -> Self {
        Self::Decimal(input.into())
    }
}

impl From<i128> for PrimitiveValue {
    fn from(input: i128) -> Self {
        Self::Decimal(input.into())
    }
}

impl From<isize> for PrimitiveValue {
    fn from(input: isize) -> Self {
        Self::Decimal(input.into())
    }
}

#[cfg(feature = "rust_decimal")]
impl From<rust_decimal::Decimal> for PrimitiveValue {
    fn from(input: rust_decimal::Decimal) -> Self {
        Self::Decimal(input.into())
    }
}

impl From<f32> for PrimitiveValue {
    fn from(input: f32) -> Self {
        Self::Float(input.into())
    }
}

impl From<Float> for PrimitiveValue {
    fn from(input: Float) -> Self {
        Self::Float(input.into())
    }
}

impl From<f64> for PrimitiveValue {
    fn from(input: f64) -> Self {
        Self::Double(input.into())
    }
}

impl From<Double> for PrimitiveValue {
    fn from(input: Double) -> Self {
        Self::Double(input.into())
    }
}

#[cfg(feature = "jiff")]
impl From<jiff::SignedDuration> for PrimitiveValue {
    fn from(input: jiff::SignedDuration) -> Self {
        Self::Duration(input)
    }
}

#[cfg(feature = "jiff")]
impl From<jiff::civil::DateTime> for PrimitiveValue {
    fn from(input: jiff::civil::DateTime) -> Self {
        Self::DateTime(input)
    }
}

#[cfg(feature = "jiff")]
impl From<jiff::civil::Time> for PrimitiveValue {
    fn from(input: jiff::civil::Time) -> Self {
        Self::Time(input)
    }
}

#[cfg(feature = "jiff")]
impl From<jiff::civil::Date> for PrimitiveValue {
    fn from(input: jiff::civil::Date) -> Self {
        Self::Date(input)
    }
}

#[cfg(feature = "alloc")]
impl From<Vec<u8>> for PrimitiveValue {
    fn from(input: Vec<u8>) -> Self {
        Self::Base64Binary(input)
    }
}

#[cfg(feature = "jiff")]
impl TryFrom<jiff::Span> for PrimitiveValue {
    type Error = jiff::Error;

    fn try_from(input: jiff::Span) -> Result<Self, Self::Error> {
        Ok(Self::Duration(jiff::SignedDuration::try_from(input)?))
    }
}

#[cfg(feature = "serde")]
impl From<PrimitiveValue> for serde_json::Value {
    fn from(input: PrimitiveValue) -> Self {
        input.into_json()
    }
}

#[cfg(feature = "serde")]
impl From<&PrimitiveValue> for serde_json::Value {
    fn from(input: &PrimitiveValue) -> Self {
        input.clone().into_json()
    }
}

#[cfg(feature = "bson")]
impl From<PrimitiveValue> for bson::Bson {
    fn from(input: PrimitiveValue) -> Self {
        input.into_bson()
    }
}
