// This is free and unencumbered software released into the public domain.

use borsh::{BorshDeserialize, BorshSerialize};

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
pub struct BorshTermId(i32);

impl BorshTermId {
    pub fn new(term_id: i32) -> Self {
        Self(term_id)
    }

    pub fn is_valid(&self) -> bool {
        !self.is_invalid()
    }

    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}

impl From<i32> for BorshTermId {
    fn from(term_id: i32) -> Self {
        Self(term_id)
    }
}

impl Into<i32> for BorshTermId {
    fn into(self) -> i32 {
        self.0
    }
}

impl TryFrom<usize> for BorshTermId {
    type Error = ();

    fn try_from(term_id: usize) -> Result<Self, Self::Error> {
        if term_id <= i32::MAX as usize {
            Ok(Self(term_id as i32))
        } else {
            Err(()) // overflow
        }
    }
}
