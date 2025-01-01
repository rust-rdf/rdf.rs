// This is free and unencumbered software released into the public domain.

use crate::{BorshTermId, BorshTriple};
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
pub struct BorshQuad {
    pub subject: BorshTermId,
    pub predicate: BorshTermId,
    pub object: BorshTermId,
    pub context: BorshTermId,
}

impl BorshQuad {
    pub fn new(
        subject: BorshTermId,
        predicate: BorshTermId,
        object: BorshTermId,
        context: BorshTermId,
    ) -> Self {
        Self {
            subject,
            predicate,
            object,
            context,
        }
    }
}

impl From<BorshTriple> for BorshQuad {
    fn from(triple: BorshTriple) -> Self {
        Self::new(
            triple.subject,
            triple.predicate,
            triple.object,
            BorshTermId::default(),
        )
    }
}

impl From<(BorshTermId, BorshTermId, BorshTermId)> for BorshQuad {
    fn from((subject, predicate, object): (BorshTermId, BorshTermId, BorshTermId)) -> Self {
        Self::new(subject, predicate, object, BorshTermId::default())
    }
}

impl From<(BorshTermId, BorshTermId, BorshTermId, BorshTermId)> for BorshQuad {
    fn from(
        (subject, predicate, object, context): (BorshTermId, BorshTermId, BorshTermId, BorshTermId),
    ) -> Self {
        Self::new(subject, predicate, object, context)
    }
}

impl From<(BorshTermId, BorshTermId, BorshTermId, Option<BorshTermId>)> for BorshQuad {
    fn from(
        (subject, predicate, object, context): (
            BorshTermId,
            BorshTermId,
            BorshTermId,
            Option<BorshTermId>,
        ),
    ) -> Self {
        Self::new(subject, predicate, object, context.unwrap_or_default())
    }
}
