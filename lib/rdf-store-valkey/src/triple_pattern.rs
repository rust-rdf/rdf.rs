// This is free and unencumbered software released into the public domain.

use crate::{ValkeyTerm, ValkeyTripleId};
use alloc::string::ToString;
use rdf_model::{HeapTerm, QuadPattern};

/// A triple statement pattern for matching triples in Valkey.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ValkeyTriplePattern {
    pub(crate) glob: ValkeyTripleId,
    pub(crate) matcher: QuadPattern<ValkeyTerm>,
}

impl ValkeyTriplePattern {
    pub fn is_constant(&self) -> bool {
        self.matcher.is_constant()
    }
}

impl<T> From<&T> for ValkeyTriplePattern
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<QuadPattern<ValkeyTerm>> for ValkeyTriplePattern {
    fn from(input: QuadPattern<ValkeyTerm>) -> Self {
        let (s, p, o, _) = input.into_inner();
        Self {
            glob: ValkeyTripleId(
                s.as_ref().and_then(|s| s.try_into().ok()),
                p.as_ref().and_then(|p| p.try_into().ok()),
                o.as_ref().and_then(|o| o.try_into().ok()),
            ),
            matcher: QuadPattern::new(
                s.map(|s| s.into()),
                p.map(|p| p.into()),
                o.map(|o| o.into()),
                None,
            ),
        }
    }
}

impl From<QuadPattern<HeapTerm>> for ValkeyTriplePattern {
    fn from(input: QuadPattern<HeapTerm>) -> Self {
        let (s, p, o, _) = input.into_inner();
        Self {
            glob: ValkeyTripleId(
                s.as_ref().map(|s| s.into()),
                p.as_ref().map(|p| p.into()),
                o.as_ref().map(|o| o.into()),
            ),
            matcher: QuadPattern::new(
                s.map(|s| s.into()),
                p.map(|p| p.into()),
                o.map(|o| o.into()),
                None,
            ),
        }
    }
}

impl From<ValkeyTriplePattern> for fred::bytes_utils::Str {
    fn from(input: ValkeyTriplePattern) -> Self {
        input.glob.to_string().into()
    }
}

impl From<ValkeyTriplePattern> for fred::types::Value {
    fn from(input: ValkeyTriplePattern) -> Self {
        input.glob.into()
    }
}

impl From<&ValkeyTriplePattern> for fred::types::Value {
    fn from(input: &ValkeyTriplePattern) -> Self {
        input.glob.clone().into()
    }
}
