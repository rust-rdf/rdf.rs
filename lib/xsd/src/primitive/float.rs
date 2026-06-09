// This is free and unencumbered software released into the public domain.

use crate::{ParseError, ParseFloatError};
use core::str::FromStr;
use decorum::Total;

/// Rust type for representing values of the `xsd:float` datatype.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Float(Total<f32>);

impl Float {
    pub fn as_f32(&self) -> f32 {
        self.0.into_inner()
    }

    pub fn as_f64(&self) -> f64 {
        self.0.into_inner() as _
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Option<serde_json::Value> {
        Some(self.clone().into_json())
    }

    #[cfg(feature = "serde")]
    pub fn into_json(self) -> serde_json::Value {
        serde_json::Number::from_f64(self.as_f64())
            .map(serde_json::Value::Number)
            .unwrap()
    }

    #[cfg(feature = "bson")]
    pub fn to_bson(&self) -> Option<bson::Bson> {
        Some(self.clone().into_bson())
    }

    #[cfg(feature = "bson")]
    pub fn into_bson(self) -> bson::Bson {
        bson::Bson::Double(self.as_f64())
    }

    pub fn into_inner(self) -> f32 {
        self.0.into_inner()
    }
}

impl From<f32> for Float {
    fn from(input: f32) -> Self {
        Self(input.into())
    }
}

impl From<f64> for Float {
    fn from(input: f64) -> Self {
        Self((input as f32).into())
    }
}

impl From<i8> for Float {
    fn from(input: i8) -> Self {
        Self((input as f32).into())
    }
}

impl From<i16> for Float {
    fn from(input: i16) -> Self {
        Self((input as f32).into())
    }
}

impl From<i32> for Float {
    fn from(input: i32) -> Self {
        Self((input as f32).into())
    }
}

impl From<i64> for Float {
    fn from(input: i64) -> Self {
        Self((input as f32).into())
    }
}

impl From<i128> for Float {
    fn from(input: i128) -> Self {
        Self((input as f32).into())
    }
}

impl From<isize> for Float {
    fn from(input: isize) -> Self {
        Self((input as f32).into())
    }
}

impl From<Float> for f32 {
    fn from(input: Float) -> Self {
        input.0.into_inner() as _
    }
}

impl From<&Float> for f32 {
    fn from(input: &Float) -> Self {
        input.0.into_inner() as _
    }
}

impl From<Float> for f64 {
    fn from(input: Float) -> Self {
        input.0.into_inner() as _
    }
}

impl From<&Float> for f64 {
    fn from(input: &Float) -> Self {
        input.0.into_inner() as _
    }
}

impl FromStr for Float {
    type Err = ParseFloatError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input.parse::<Total<f32>>().map(Self)
    }
}

impl core::fmt::Display for Float {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let v = self.as_f32();
        if v.is_nan() {
            // XML Schema lexical form for NaN
            return write!(f, "NaN");
        }
        if v.is_infinite() {
            // XML Schema lexical forms for infinities: INF and -INF
            return if v.is_sign_positive() {
                write!(f, "INF")
            } else {
                write!(f, "-INF")
            };
        }
        // Finite values: delegate to f32's formatting (preserves sign of zero)
        write!(f, "{}", v)
    }
}

#[cfg(feature = "borsh")]
impl borsh::BorshSerialize for Float {
    fn serialize<W: borsh::io::Write>(&self, writer: &mut W) -> borsh::io::Result<()> {
        let value = self.as_f32();
        value.serialize(writer)
    }
}

#[cfg(feature = "borsh")]
impl borsh::BorshDeserialize for Float {
    fn deserialize_reader<R: borsh::io::Read>(reader: &mut R) -> borsh::io::Result<Self> {
        let value = f32::deserialize_reader(reader)?;
        Ok(Float(value.into()))
    }
}

#[cfg(feature = "bson")]
impl From<Float> for bson::Bson {
    fn from(input: Float) -> Self {
        input.into_bson()
    }
}
