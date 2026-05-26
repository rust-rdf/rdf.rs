// This is free and unencumbered software released into the public domain.

use crate::{ParseDoubleError, ParseError};
use core::str::FromStr;
use decorum::Total;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Double(Total<f64>);

impl Double {
    pub fn as_f32(&self) -> f32 {
        self.0.into_inner() as _
    }

    pub fn as_f64(&self) -> f64 {
        self.0.into_inner()
    }
}

impl From<f32> for Double {
    fn from(input: f32) -> Self {
        Self((input as f64).into())
    }
}

impl From<f64> for Double {
    fn from(input: f64) -> Self {
        Self(input.into())
    }
}

impl From<i8> for Double {
    fn from(input: i8) -> Self {
        Self((input as f64).into())
    }
}

impl From<i16> for Double {
    fn from(input: i16) -> Self {
        Self((input as f64).into())
    }
}

impl From<i32> for Double {
    fn from(input: i32) -> Self {
        Self((input as f64).into())
    }
}

impl From<i64> for Double {
    fn from(input: i64) -> Self {
        Self((input as f64).into())
    }
}

impl From<i128> for Double {
    fn from(input: i128) -> Self {
        Self((input as f64).into())
    }
}

impl From<isize> for Double {
    fn from(input: isize) -> Self {
        Self((input as f64).into())
    }
}

impl FromStr for Double {
    type Err = ParseDoubleError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input
            .parse::<Total<f64>>()
            .map(Self)
            .map_err(|_| ParseError)
    }
}

impl core::fmt::Display for Double {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let v = self.as_f64();
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
        // Finite values: delegate to f64's formatting (preserves sign of zero)
        write!(f, "{}", v)
    }
}

#[cfg(feature = "borsh")]
impl borsh::BorshSerialize for Double {
    fn serialize<W: borsh::io::Write>(&self, writer: &mut W) -> borsh::io::Result<()> {
        let value = self.as_f64();
        value.serialize(writer)
    }
}

#[cfg(feature = "borsh")]
impl borsh::BorshDeserialize for Double {
    fn deserialize_reader<R: borsh::io::Read>(reader: &mut R) -> borsh::io::Result<Self> {
        let value = f64::deserialize_reader(reader)?;
        Ok(Double(value.into()))
    }
}
