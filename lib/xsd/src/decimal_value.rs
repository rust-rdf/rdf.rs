// This is free and unencumbered software released into the public domain.

use crate::{
    DecimalType,
    derived::{Byte, Int, Integer, Long, Short},
    primitive::Decimal,
};
use strum_macros::Display;

/// Value representation for `xsd:decimal` datatypes.
///
/// [`Display`](core::fmt::Display) writes the contained number without a datatype
/// label, including when the `alloc` feature is disabled.
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
    #[strum(transparent)]
    Decimal(Decimal),

    /// See: <https://www.w3.org/TR/xmlschema-2/#integer>
    #[strum(transparent)]
    Integer(Integer),

    /// See: <https://www.w3.org/TR/xmlschema-2/#long>
    #[strum(transparent)]
    Long(Long),

    /// See: <https://www.w3.org/TR/xmlschema-2/#int>
    #[strum(transparent)]
    Int(Int),

    /// See: <https://www.w3.org/TR/xmlschema-2/#short>
    #[strum(transparent)]
    Short(Short),

    /// See: <https://www.w3.org/TR/xmlschema-2/#byte>
    #[strum(transparent)]
    Byte(Byte),
}

impl DecimalValue {
    pub fn as_f64(&self) -> f64 {
        use DecimalValue::*;
        match self {
            Decimal(d) => d.as_f64(),
            Integer(n) => n.as_f64(),
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

    /// Upcasts this value to its immediate base type, preserving its exact number.
    ///
    /// Returns `None` for a decimal (already the base type), or when an integer
    /// cannot be represented exactly by the decimal backend. The current backend
    /// supports integer magnitudes up to `79228162514264337593543950335`.
    /// Although XSD decimal contains the integer value space, this bounded Rust
    /// representation does not. Out-of-range widening returns `None` rather than
    /// panicking. Requires neither allocation nor `std`.
    ///
    /// ```
    /// use xsd::DecimalValue;
    /// assert_eq!(DecimalValue::Byte(42).widen(), Some(DecimalValue::Short(42)));
    /// assert!(DecimalValue::from(i128::MAX).widen().is_none());
    /// ```
    pub fn widen(&self) -> Option<Self> {
        use DecimalValue::*;
        match self {
            Decimal(_) => None, // already the widest primitive base type
            Integer(n) => {
                use core::fmt::Write;
                // Avoid the backend's infallible integer conversion, which can
                // panic. An i128 needs at most 39 digits and one minus sign.
                let mut lexical = heapless::String::<40>::new();
                write!(&mut lexical, "{n}").ok()?;
                let decimal = lexical.parse::<crate::primitive::Decimal>().ok()?;
                // Guard exactness as well as range if backend behavior changes.
                let recovered = i128::try_from(&decimal).ok()?;
                (crate::derived::Integer::from(recovered) == *n).then_some(Decimal(decimal))
            },
            Long(n) => Some(Integer(n.into())),
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
            Decimal(d) => i128::try_from(d).ok().map(|z| Self::Integer(z.into())),
            Integer(n) => i64::try_from(*n).ok().map(Self::Long),
            Long(n) => i32::try_from(*n).ok().map(Self::Int),
            Int(n) => i16::try_from(*n).ok().map(Self::Short),
            Short(n) => i8::try_from(*n).ok().map(Self::Byte),
            Byte(_) => None, // already the narrowest derived type
        }
    }

    /// Clones and converts this value using [`Self::into_json`]. Requires `serde`.
    /// Integer conversion always returns `Some`, using strings outside `i64`.
    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Option<serde_json::Value> {
        Some(self.clone().into_json())
    }

    /// Converts to an untagged JSON value. Requires `serde`.
    ///
    /// Integer-family values use exact JSON integer numbers within the `i64`
    /// range; larger `Integer` values use decimal strings. This replaces the
    /// former lossy `f64` encoding of `Integer`. Consumers that parse all JSON
    /// numbers as binary64 may still lose precision beyond 53 bits.
    /// Datatype identity and original lexical spelling are not retained; retain
    /// the datatype separately to parse the number or string back into XSD.
    /// This differs from the enum's derived Serde serialization.
    #[cfg(feature = "serde")]
    pub fn into_json(self) -> serde_json::Value {
        use DecimalValue::*;
        match self {
            Decimal(r) => r.as_f64().into(), // TODO: string
            Integer(z) => {
                use alloc::string::ToString;
                match z.to_i64() {
                    Some(n) => n.into(),
                    None => serde_json::Value::String(z.to_string()),
                }
            },
            Long(z) => z.into(),
            Int(z) => z.into(),
            Short(z) => z.into(),
            Byte(z) => z.into(),
        }
    }

    #[cfg(feature = "bson")]
    pub fn to_bson(&self) -> Option<bson::Bson> {
        Some(self.clone().into_bson())
    }

    #[cfg(feature = "bson")]
    pub fn into_bson(self) -> bson::Bson {
        use DecimalValue::*;
        use bson::{Bson, Decimal128};
        match self {
            Decimal(r) => r.into_bson().unwrap(),
            Integer(z) if z.to_i64().is_some() => Bson::Int64(z.to_i64().unwrap() as _),
            Integer(z) => {
                use alloc::string::ToString;
                use core::str::FromStr;
                Bson::Decimal128(Decimal128::from_str(z.to_string().as_str()).unwrap()) // FIXME
            },
            Long(n) => Bson::Int64(n),
            Int(n) => Bson::Int32(n),
            Short(n) => Bson::Int32(n.into()),
            Byte(n) => Bson::Int32(n.into()),
        }
    }
}

impl<T> From<&T> for DecimalValue
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
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

impl From<Integer> for DecimalValue {
    fn from(input: Integer) -> Self {
        Self::Integer(input)
    }
}

impl From<Decimal> for DecimalValue {
    fn from(input: Decimal) -> Self {
        Self::Decimal(input)
    }
}

#[cfg(feature = "rust_decimal")]
impl From<rust_decimal::Decimal> for DecimalValue {
    fn from(input: rust_decimal::Decimal) -> Self {
        Self::Decimal(input.into())
    }
}

impl TryFrom<f32> for DecimalValue {
    type Error = valuand::DecimalError;

    fn try_from(input: f32) -> Result<Self, Self::Error> {
        Ok(Self::Decimal(input.try_into()?))
    }
}

impl TryFrom<f64> for DecimalValue {
    type Error = valuand::DecimalError;

    fn try_from(input: f64) -> Result<Self, Self::Error> {
        Ok(Self::Decimal(input.try_into()?))
    }
}

#[cfg(feature = "serde")]
impl From<DecimalValue> for serde_json::Value {
    fn from(input: DecimalValue) -> Self {
        input.into_json()
    }
}

#[cfg(feature = "serde")]
impl From<&DecimalValue> for serde_json::Value {
    fn from(input: &DecimalValue) -> Self {
        input.clone().into_json()
    }
}

#[cfg(feature = "bson")]
impl From<DecimalValue> for bson::Bson {
    fn from(input: DecimalValue) -> Self {
        bson::Bson::Double(input.as_f64())
    }
}
