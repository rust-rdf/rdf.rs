// This is free and unencumbered software released into the public domain.

use crate::{ParseBooleanError, ParseError};
use core::str::FromStr;

/// Rust type for representing values of the `xsd:boolean` datatype.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#boolean>
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct Boolean(pub bool);

impl Boolean {
    pub const TRUE: Boolean = Boolean(true);
    pub const FALSE: Boolean = Boolean(false);

    pub fn is_false(&self) -> bool {
        self.0 == false
    }

    pub fn is_true(&self) -> bool {
        self.0 == true
    }

    pub fn as_bool(&self) -> bool {
        self.0
    }

    pub fn to_bool(&self) -> bool {
        self.0
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Option<serde_json::Value> {
        Some(self.clone().into_json())
    }

    #[cfg(feature = "serde")]
    pub fn into_json(self) -> serde_json::Value {
        serde_json::Value::Bool(self.as_bool())
    }

    #[cfg(feature = "bson")]
    pub fn to_bson(&self) -> Option<bson::Bson> {
        Some(self.clone().into_bson())
    }

    #[cfg(feature = "bson")]
    pub fn into_bson(self) -> bson::Bson {
        bson::Bson::Boolean(self.as_bool())
    }

    pub fn into_inner(self) -> bool {
        self.0
    }
}

impl core::fmt::Display for Boolean {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            false => write!(f, "false"),
            true => write!(f, "true"),
        }
    }
}

impl From<bool> for Boolean {
    fn from(input: bool) -> Self {
        Self(input)
    }
}

impl From<Boolean> for bool {
    fn from(input: Boolean) -> Self {
        input.0
    }
}

impl From<&Boolean> for bool {
    fn from(input: &Boolean) -> Self {
        input.0
    }
}

impl FromStr for Boolean {
    type Err = ParseBooleanError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "0" | "false" => Ok(Self(false)),
            "1" | "true" => Ok(Self(true)),
            _ => Err(ParseError),
        }
    }
}

#[cfg(feature = "bson")]
impl From<Boolean> for bson::Bson {
    fn from(input: Boolean) -> Self {
        input.into_bson()
    }
}
