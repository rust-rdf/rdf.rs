// This is free and unencumbered software released into the public domain.

use crate::{
    DecimalValue, PrimitiveValue, Type,
    primitives::{Boolean, Decimal, Double, Float},
};
use strum_macros::Display;

#[cfg(feature = "jiff")]
use crate::primitives::{Date, DateTime, Duration, Time};

#[cfg(feature = "alloc")]
use ::alloc::{borrow::Cow, string::String};

/// An XML Schema value.
///
/// See: https://www.w3.org/TR/xmlschema-2/#built-in-dataValues
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

#[cfg(feature = "alloc")]
impl From<&String> for Value {
    fn from(input: &String) -> Self {
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
