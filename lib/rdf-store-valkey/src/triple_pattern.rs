// This is free and unencumbered software released into the public domain.

use alloc::{format, string::String};
use rdf_model::{HeapTerm, QuadPattern};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ValkeyTriplePattern {
    pub(crate) glob: String,
    pub(crate) matcher: QuadPattern<HeapTerm>,
}

impl From<QuadPattern<HeapTerm>> for ValkeyTriplePattern {
    fn from(input: QuadPattern<HeapTerm>) -> Self {
        let (s, p, o, g) = input.into_inner();
        Self {
            glob: format!(
                "{}:{}:{}",
                s.as_ref()
                    .map(|term| term.to_b3().to_hex()[0..16].to_lowercase())
                    .unwrap_or_else(|| "*".into()),
                p.as_ref()
                    .map(|term| term.to_b3().to_hex()[0..16].to_lowercase())
                    .unwrap_or_else(|| "*".into()),
                o.as_ref()
                    .map(|term| term.to_b3().to_hex()[0..16].to_lowercase())
                    .unwrap_or_else(|| "*".into()),
            ),
            matcher: QuadPattern::new(s, p, o, g),
        }
    }
}

impl From<&ValkeyTriplePattern> for fred::types::Value {
    fn from(input: &ValkeyTriplePattern) -> Self {
        fred::types::Value::String(input.glob.clone().into())
    }
}
