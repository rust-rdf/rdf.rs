// This is free and unencumbered software released into the public domain.

use crate::{DecimalType, primitives::Decimal};
use strum_macros::Display;

#[cfg(feature = "alloc")]
use ::alloc::format;

/// The XML Schema `xsd:decimal` datatypes.
///
/// See: https://www.w3.org/TR/xmlschema-2/#built-in-datatypes
#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
// #[cfg_attr(
//     feature = "borsh",
//     derive(borsh::BorshSerialize, borsh::BorshDeserialize)
// )]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DecimalValue {
    /// See: https://www.w3.org/TR/xmlschema-2/#decimal
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Decimal(Decimal),

    /// See: https://www.w3.org/TR/xmlschema-2/#integer
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Integer(i128),

    /// See: https://www.w3.org/TR/xmlschema-2/#long
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Long(i64),

    /// See: https://www.w3.org/TR/xmlschema-2/#int
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Int(i32),

    /// See: https://www.w3.org/TR/xmlschema-2/#short
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Short(i16),

    /// See: https://www.w3.org/TR/xmlschema-2/#byte
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Byte(i8),
}

impl DecimalValue {
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
}

impl From<i8> for DecimalValue {
    fn from(input: i8) -> Self {
        Self::Byte(input)
    }
}

impl From<i16> for DecimalValue {
    fn from(input: i16) -> Self {
        Self::Short(input)
    }
}

impl From<i32> for DecimalValue {
    fn from(input: i32) -> Self {
        Self::Int(input)
    }
}

impl From<i64> for DecimalValue {
    fn from(input: i64) -> Self {
        Self::Long(input)
    }
}

#[cfg(feature = "rust_decimal")]
impl From<i128> for DecimalValue {
    fn from(input: i128) -> Self {
        Self::Decimal(input.into())
    }
}

#[cfg(feature = "rust_decimal")]
impl From<isize> for DecimalValue {
    fn from(input: isize) -> Self {
        Self::Decimal(input.into())
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

#[cfg(feature = "rust_decimal")]
impl TryFrom<f32> for DecimalValue {
    type Error = ();

    fn try_from(input: f32) -> Result<Self, Self::Error> {
        Ok(Self::Decimal(input.try_into()?))
    }
}

#[cfg(feature = "rust_decimal")]
impl TryFrom<f64> for DecimalValue {
    type Error = ();

    fn try_from(input: f64) -> Result<Self, Self::Error> {
        Ok(Self::Decimal(input.try_into()?))
    }
}
