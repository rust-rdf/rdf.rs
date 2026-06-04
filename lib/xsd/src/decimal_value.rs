// This is free and unencumbered software released into the public domain.

use crate::{
    DecimalType, ParseDecimalError,
    primitive::{Byte, Decimal, Int, Integer, Long, Short},
};
use strum_macros::Display;

#[cfg(feature = "alloc")]
use ::alloc::format;

/// Value representation for `xsd:decimal` datatypes.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#built-in-datatypes>
#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DecimalValue {
    /// See: <https://www.w3.org/TR/xmlschema-2/#decimal>
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Decimal(Decimal),

    /// See: <https://www.w3.org/TR/xmlschema-2/#integer>
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Integer(Integer),

    /// See: <https://www.w3.org/TR/xmlschema-2/#long>
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Long(Long),

    /// See: <https://www.w3.org/TR/xmlschema-2/#int>
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Int(Int),

    /// See: <https://www.w3.org/TR/xmlschema-2/#short>
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Short(Short),

    /// See: <https://www.w3.org/TR/xmlschema-2/#byte>
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Byte(Byte),
}

impl DecimalValue {
    pub fn as_f64(&self) -> f64 {
        use DecimalValue::*;
        match self {
            Decimal(d) => return d.as_f64(),
            Integer(n) => *n as _,
            Long(n) => *n as _,
            Int(n) => *n as _,
            Short(n) => *n as _,
            Byte(n) => *n as _,
        }
    }

    pub fn to_f64(&self) -> Option<f64> {
        Some(self.as_f64())
    }

    pub fn r#type(&self) -> DecimalType {
        use DecimalValue::*;
        match self {
            Decimal(_) => DecimalType::Decimal,
            Integer(_) => DecimalType::Integer,
            Long(_) => DecimalType::Long,
            Int(_) => DecimalType::Int,
            Short(_) => DecimalType::Short,
            Byte(_) => DecimalType::Byte,
        }
    }

    /// Upcasts the type of this value to its wider base type.
    pub fn widen(&self) -> Option<Self> {
        use DecimalValue::*;
        match self {
            Decimal(_) => None, // already the widest primitive base type
            Integer(n) => (*n).try_into().ok(),
            Long(n) => Some(Integer(*n as _)),
            Int(n) => Some(Long(*n as _)),
            Short(n) => Some(Int(*n as _)),
            Byte(n) => Some(Short(*n as _)),
        }
    }

    /// Downcasts the type of this value to a compatible narrower derived
    /// type, if feasible.
    pub fn narrow(&self) -> Option<Self> {
        use DecimalValue::*;
        match self {
            Decimal(d) if !d.is_integer() => None,
            Decimal(d) => i128::try_from(d).ok().map(Integer),
            Integer(n) => i64::try_from(*n).ok().map(Long),
            Long(n) => i32::try_from(*n).ok().map(Int),
            Int(n) => i16::try_from(*n).ok().map(Short),
            Short(n) => i8::try_from(*n).ok().map(Byte),
            Byte(_) => None, // already the narrowest derived type
        }
    }
}

impl From<i8> for DecimalValue {
    fn from(input: i8) -> Self {
        Self::Byte(input.into())
    }
}

impl From<i16> for DecimalValue {
    fn from(input: i16) -> Self {
        Self::Short(input.into())
    }
}

impl From<i32> for DecimalValue {
    fn from(input: i32) -> Self {
        Self::Int(input.into())
    }
}

impl From<i64> for DecimalValue {
    fn from(input: i64) -> Self {
        Self::Long(input.into())
    }
}

impl From<i128> for DecimalValue {
    fn from(input: i128) -> Self {
        Self::Integer(input.into())
    }
}

impl From<isize> for DecimalValue {
    fn from(input: isize) -> Self {
        Self::Integer((input as i128).into()) // TODO
    }
}

impl From<Decimal> for DecimalValue {
    fn from(input: Decimal) -> Self {
        Self::Decimal(input)
    }
}

impl From<&Decimal> for DecimalValue {
    fn from(input: &Decimal) -> Self {
        Self::Decimal(input.clone())
    }
}

#[cfg(feature = "rust_decimal")]
impl From<rust_decimal::Decimal> for DecimalValue {
    fn from(input: rust_decimal::Decimal) -> Self {
        Self::Decimal(input.into())
    }
}

#[cfg(feature = "rust_decimal")]
impl From<&rust_decimal::Decimal> for DecimalValue {
    fn from(input: &rust_decimal::Decimal) -> Self {
        Self::Decimal(input.clone().into())
    }
}

impl TryFrom<f32> for DecimalValue {
    type Error = ParseDecimalError;

    fn try_from(input: f32) -> Result<Self, Self::Error> {
        Ok(Self::Decimal(input.try_into()?))
    }
}

impl TryFrom<f64> for DecimalValue {
    type Error = ParseDecimalError;

    fn try_from(input: f64) -> Result<Self, Self::Error> {
        Ok(Self::Decimal(input.try_into()?))
    }
}

#[cfg(feature = "serde")]
impl From<DecimalValue> for serde_json::Value {
    fn from(input: DecimalValue) -> Self {
        (&input).into()
    }
}

#[cfg(feature = "serde")]
impl From<&DecimalValue> for serde_json::Value {
    fn from(input: &DecimalValue) -> Self {
        use DecimalValue::*;
        match input {
            Decimal(d) => d.as_f64().into(),
            Integer(n) => (*n as f64).into(),
            Long(n) => (*n).into(),
            Int(n) => (*n).into(),
            Short(n) => (*n).into(),
            Byte(n) => (*n).into(),
        }
    }
}
