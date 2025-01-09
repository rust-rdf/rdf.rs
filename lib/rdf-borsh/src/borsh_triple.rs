// This is free and unencumbered software released into the public domain.

use crate::{BorshQuad, BorshTermId};
use borsh::{BorshDeserialize, BorshSerialize};
use num_integer::Integer;

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
pub struct BorshTriple<T: Integer> {
    pub subject: BorshTermId<T>,
    pub predicate: BorshTermId<T>,
    pub object: BorshTermId<T>,
}

impl<T: Integer> BorshTriple<T> {
    pub fn new(subject: BorshTermId<T>, predicate: BorshTermId<T>, object: BorshTermId<T>) -> Self {
        Self {
            subject,
            predicate,
            object,
        }
    }
}

impl<T: Integer + Default> From<BorshQuad<T>> for BorshTriple<T> {
    fn from(triple: BorshQuad<T>) -> Self {
        Self::new(triple.subject, triple.predicate, triple.object)
    }
}

impl<T: Integer> From<(BorshTermId<T>, BorshTermId<T>, BorshTermId<T>)> for BorshTriple<T> {
    fn from(
        (subject, predicate, object): (BorshTermId<T>, BorshTermId<T>, BorshTermId<T>),
    ) -> Self {
        Self::new(subject, predicate, object)
    }
}
