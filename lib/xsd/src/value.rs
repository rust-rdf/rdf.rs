// This is free and unencumbered software released into the public domain.

use crate::{
    DecimalValue, PrimitiveValue, Type,
    primitive::{Boolean, Decimal, Double, Float},
};
use strum_macros::Display;

#[cfg(feature = "jiff")]
use crate::primitive::{Date, DateTime, Duration, Time};

#[cfg(feature = "alloc")]
use ::alloc::{borrow::Cow, string::String};

/// An XSD value.
///
/// Currently supports the primitive datatypes and the derived `xsd:decimal`
/// datatypes.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#built-in-dataValues>
#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
// #[cfg_attr(
//     feature = "borsh",
//     derive(borsh::BorshSerialize, borsh::BorshDeserialize)
// )]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value {
    Primitive(PrimitiveValue),

    Decimal(DecimalValue),
}

impl Value {
    #[cfg(feature = "alloc")]
    pub fn string(value: impl Into<String>) -> Self {
        Self::Primitive(value.into().into())
    }

    pub fn boolean(value: impl Into<Boolean>) -> Self {
        Self::Primitive(value.into().into())
    }

    pub fn decimal(value: impl Into<Decimal>) -> Self {
        Self::Decimal(value.into().into())
    }

    pub fn float(value: impl Into<Float>) -> Self {
        Self::Primitive(value.into().into())
    }

    pub fn double(value: impl Into<Double>) -> Self {
        Self::Primitive(value.into().into())
    }

    #[cfg(feature = "jiff")]
    pub fn duration(value: impl Into<Duration>) -> Self {
        Self::Primitive(value.into().into())
    }

    #[cfg(feature = "jiff")]
    pub fn datetime(value: impl Into<DateTime>) -> Self {
        Self::Primitive(value.into().into())
    }

    #[cfg(feature = "jiff")]
    pub fn time(value: impl Into<Time>) -> Self {
        Self::Primitive(value.into().into())
    }

    #[cfg(feature = "jiff")]
    pub fn date(value: impl Into<Date>) -> Self {
        Self::Primitive(value.into().into())
    }

    pub fn r#type(&self) -> Type {
        use Value::*;
        match self {
            Primitive(v) => Type::Primitive(v.r#type()),
            Decimal(v) => Type::Decimal(v.r#type()),
        }
    }

    pub fn as_decimal(&self) -> &DecimalValue {
        self.to_decimal().expect("value must be a decimal")
    }

    pub fn as_primitive(&self) -> &PrimitiveValue {
        self.to_primitive().expect("value must be a primitive")
    }

    pub fn to_decimal(&self) -> Option<&DecimalValue> {
        match self {
            Self::Decimal(val) => Some(&val),
            _ => None,
        }
    }

    pub fn to_primitive(&self) -> Option<&PrimitiveValue> {
        match self {
            Self::Primitive(val) => Some(&val),
            _ => None,
        }
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Option<serde_json::Value> {
        Some(self.clone().into_json())
    }

    #[cfg(feature = "serde")]
    pub fn into_json(self) -> serde_json::Value {
        match self {
            Self::Primitive(val) => val.into_json(),
            Self::Decimal(val) => val.into_json(),
        }
    }

    #[cfg(feature = "bson")]
    pub fn to_bson(&self) -> Option<bson::Bson> {
        Some(self.clone().into_bson())
    }

    #[cfg(feature = "bson")]
    pub fn into_bson(self) -> bson::Bson {
        match self {
            Self::Primitive(val) => val.into_bson(),
            Self::Decimal(val) => val.into_bson(),
        }
    }
}

impl<T> From<&T> for Value
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<DecimalValue> for Value {
    fn from(input: DecimalValue) -> Self {
        Self::Decimal(input)
    }
}

impl From<PrimitiveValue> for Value {
    fn from(input: PrimitiveValue) -> Self {
        Self::Primitive(input)
    }
}

impl From<&'static str> for Value {
    fn from(input: &'static str) -> Self {
        Self::Primitive(input.into())
    }
}

#[cfg(feature = "alloc")]
impl From<Cow<'_, str>> for Value {
    fn from(input: Cow<'_, str>) -> Self {
        Self::Primitive(input.into())
    }
}

#[cfg(feature = "alloc")]
impl From<String> for Value {
    fn from(input: String) -> Self {
        Self::Primitive(input.into())
    }
}

impl From<bool> for Value {
    fn from(input: bool) -> Self {
        Self::Primitive(input.into())
    }
}

impl From<Boolean> for Value {
    fn from(input: Boolean) -> Self {
        Self::Primitive(input.into())
    }
}

impl From<i8> for Value {
    fn from(input: i8) -> Self {
        Self::Primitive(input.into())
    }
}

impl From<i16> for Value {
    fn from(input: i16) -> Self {
        Self::Primitive(input.into())
    }
}

impl From<i32> for Value {
    fn from(input: i32) -> Self {
        Self::Primitive(input.into())
    }
}

impl From<i64> for Value {
    fn from(input: i64) -> Self {
        Self::Primitive(input.into())
    }
}

impl From<i128> for Value {
    fn from(input: i128) -> Self {
        Self::Primitive(input.into())
    }
}

impl From<isize> for Value {
    fn from(input: isize) -> Self {
        Self::Primitive(input.into())
    }
}

impl From<Decimal> for Value {
    fn from(input: Decimal) -> Self {
        Self::Decimal(input.into())
    }
}

impl From<f32> for Value {
    fn from(input: f32) -> Self {
        Self::Primitive(input.into())
    }
}

impl From<Float> for Value {
    fn from(input: Float) -> Self {
        Self::Primitive(input.into())
    }
}

impl From<f64> for Value {
    fn from(input: f64) -> Self {
        Self::Primitive(input.into())
    }
}

impl From<Double> for Value {
    fn from(input: Double) -> Self {
        Self::Primitive(input.into())
    }
}

#[cfg(feature = "jiff")]
impl From<Duration> for Value {
    fn from(input: Duration) -> Self {
        Self::Primitive(input.into())
    }
}

#[cfg(feature = "jiff")]
impl From<DateTime> for Value {
    fn from(input: DateTime) -> Self {
        Self::Primitive(input.into())
    }
}

#[cfg(feature = "jiff")]
impl From<Time> for Value {
    fn from(input: Time) -> Self {
        Self::Primitive(input.into())
    }
}

#[cfg(feature = "jiff")]
impl From<Date> for Value {
    fn from(input: Date) -> Self {
        Self::Primitive(input.into())
    }
}

#[cfg(feature = "serde")]
impl From<Value> for serde_json::Value {
    fn from(input: Value) -> Self {
        input.into_json()
    }
}

#[cfg(feature = "serde")]
impl From<&Value> for serde_json::Value {
    fn from(input: &Value) -> Self {
        input.clone().into_json()
    }
}

#[cfg(feature = "bson")]
impl From<Value> for bson::Bson {
    fn from(input: Value) -> Self {
        input.into_bson()
    }
}
