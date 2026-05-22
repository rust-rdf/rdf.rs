// This is free and unencumbered software released into the public domain.

#[cfg(feature = "rust_decimal")]
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};

#[cfg(feature = "rust_decimal")]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Decimal(
    #[cfg_attr(feature = "serde", serde(with = "rust_decimal::serde::str"))] rust_decimal::Decimal,
);

#[cfg(not(feature = "rust_decimal"))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Decimal(crate::primitives::Double);

impl Decimal {
    pub fn is_integer(&self) -> bool {
        #[cfg(feature = "rust_decimal")]
        return self.0.is_integer();
        #[cfg(not(feature = "rust_decimal"))]
        return self.0.as_f64().fract() == 0.0;
    }

    pub fn as_f64(&self) -> f64 {
        return self.0.as_f64();
    }

    pub fn to_f64(&self) -> Option<f64> {
        Some(self.as_f64())
    }

    pub fn to_i128(&self) -> Option<i128> {
        if !self.is_integer() {
            return None;
        }
        #[cfg(feature = "rust_decimal")]
        return self.0.to_i128();
        #[cfg(not(feature = "rust_decimal"))]
        return Some(self.as_f64() as i128);
    }
}

impl core::fmt::Display for Decimal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<i8> for Decimal {
    fn from(input: i8) -> Self {
        Self(input.into())
    }
}

impl From<i16> for Decimal {
    fn from(input: i16) -> Self {
        Self(input.into())
    }
}

impl From<i32> for Decimal {
    fn from(input: i32) -> Self {
        Self(input.into())
    }
}

impl From<i64> for Decimal {
    fn from(input: i64) -> Self {
        Self(input.into())
    }
}

impl From<i128> for Decimal {
    fn from(input: i128) -> Self {
        Self(input.into())
    }
}

impl From<isize> for Decimal {
    fn from(input: isize) -> Self {
        Self(input.into())
    }
}

#[cfg(feature = "rust_decimal")]
impl From<rust_decimal::Decimal> for Decimal {
    fn from(input: rust_decimal::Decimal) -> Self {
        Self(input)
    }
}

#[cfg(feature = "rust_decimal")]
impl From<&rust_decimal::Decimal> for Decimal {
    fn from(input: &rust_decimal::Decimal) -> Self {
        Self(input.clone())
    }
}

impl TryFrom<f32> for Decimal {
    type Error = ();

    fn try_from(input: f32) -> Result<Self, Self::Error> {
        #[cfg(feature = "rust_decimal")]
        return Ok(Self(rust_decimal::Decimal::from_f32(input).ok_or(())?));
        #[cfg(not(feature = "rust_decimal"))]
        return Ok(Self(input.into()));
    }
}

impl TryFrom<f64> for Decimal {
    type Error = ();

    fn try_from(input: f64) -> Result<Self, Self::Error> {
        #[cfg(feature = "rust_decimal")]
        return Ok(Self(rust_decimal::Decimal::from_f64(input).ok_or(())?));
        #[cfg(not(feature = "rust_decimal"))]
        return Ok(Self(input.into()));
    }
}

impl TryFrom<Decimal> for i128 {
    type Error = ();

    fn try_from(input: Decimal) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&Decimal> for i128 {
    type Error = ();

    fn try_from(input: &Decimal) -> Result<Self, Self::Error> {
        input.to_i128().ok_or(())
    }
}
