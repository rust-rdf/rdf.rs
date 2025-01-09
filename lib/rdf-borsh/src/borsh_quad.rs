// This is free and unencumbered software released into the public domain.

use crate::{BorshTermId, BorshTriple};
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
pub struct BorshQuad<T: Integer> {
    pub context: BorshTermId<T>,
    pub subject: BorshTermId<T>,
    pub predicate: BorshTermId<T>,
    pub object: BorshTermId<T>,
}

impl<T: Integer> BorshQuad<T> {
    pub fn new(
        subject: BorshTermId<T>,
        predicate: BorshTermId<T>,
        object: BorshTermId<T>,
        context: BorshTermId<T>,
    ) -> Self {
        Self {
            context,
            subject,
            predicate,
            object,
        }
    }
}

impl<T: Integer + Default> From<BorshTriple<T>> for BorshQuad<T> {
    fn from(triple: BorshTriple<T>) -> Self {
        Self::new(
            triple.subject,
            triple.predicate,
            triple.object,
            BorshTermId::default(),
        )
    }
}

impl<T: Integer + Default> From<(BorshTermId<T>, BorshTermId<T>, BorshTermId<T>)> for BorshQuad<T> {
    fn from(
        (subject, predicate, object): (BorshTermId<T>, BorshTermId<T>, BorshTermId<T>),
    ) -> Self {
        Self::new(subject, predicate, object, BorshTermId::default())
    }
}

impl<T: Integer>
    From<(
        BorshTermId<T>,
        BorshTermId<T>,
        BorshTermId<T>,
        BorshTermId<T>,
    )> for BorshQuad<T>
{
    fn from(
        (subject, predicate, object, context): (
            BorshTermId<T>,
            BorshTermId<T>,
            BorshTermId<T>,
            BorshTermId<T>,
        ),
    ) -> Self {
        Self::new(subject, predicate, object, context)
    }
}

impl<T: Integer + Default>
    From<(
        BorshTermId<T>,
        BorshTermId<T>,
        BorshTermId<T>,
        Option<BorshTermId<T>>,
    )> for BorshQuad<T>
{
    fn from(
        (subject, predicate, object, context): (
            BorshTermId<T>,
            BorshTermId<T>,
            BorshTermId<T>,
            Option<BorshTermId<T>>,
        ),
    ) -> Self {
        Self::new(subject, predicate, object, context.unwrap_or_default())
    }
}
