// This is free and unencumbered software released into the public domain.

use borsh::{BorshDeserialize, BorshSerialize};
use num_integer::Integer;
use num_traits::FromPrimitive;

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    BorshSerialize,
    BorshDeserialize,
)]
pub struct BorshTermId<T: Integer>(T);

impl<T: Integer> BorshTermId<T> {
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl From<u16> for BorshTermId<u16> {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl FromPrimitive for BorshTermId<u16> {
    fn from_i64(n: i64) -> Option<Self> {
        if n <= u16::MAX as _ && n >= 0 {
            Some(Self(n as u16))
        } else {
            None // overflow
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        if n <= u16::MAX as _ {
            Some(Self(n as u16))
        } else {
            None // overflow
        }
    }
}
