// This is free and unencumbered software released into the public domain.

use decorum::Total;

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
