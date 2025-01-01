// This is free and unencumbered software released into the public domain.

use crate::{BorshQuad, BorshTermId};
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
pub struct BorshTriple {
    pub subject: BorshTermId,
    pub predicate: BorshTermId,
    pub object: BorshTermId,
}

impl BorshTriple {
    pub fn new(subject: BorshTermId, predicate: BorshTermId, object: BorshTermId) -> Self {
        Self {
            subject,
            predicate,
            object,
        }
    }
}

impl From<BorshQuad> for BorshTriple {
    fn from(triple: BorshQuad) -> Self {
        Self::new(triple.subject, triple.predicate, triple.object)
    }
}

impl From<(BorshTermId, BorshTermId, BorshTermId)> for BorshTriple {
    fn from((subject, predicate, object): (BorshTermId, BorshTermId, BorshTermId)) -> Self {
        Self::new(subject, predicate, object)
    }
}
