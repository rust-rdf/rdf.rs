// This is free and unencumbered software released into the public domain.

use crate::HeapStore;
use alloc::string::String;
use oxrdf::Term;
use spareval::QueryEvaluationError;
use spareval::{InternalQuad, QueryableDataset};

#[derive(Debug)]
pub struct OxrdfHeapStore {
    inner: HeapStore,
}

impl OxrdfHeapStore {
    pub fn new(inner: HeapStore) -> Self {
        Self { inner }
    }
}

impl Default for OxrdfHeapStore {
    fn default() -> Self {
        Self::new(HeapStore::default())
    }
}

impl AsRef<HeapStore> for OxrdfHeapStore {
    fn as_ref(&self) -> &HeapStore {
        &self.inner
    }
}

impl<'a> QueryableDataset<'a> for &'a OxrdfHeapStore {
    type InternalTerm = String; // TODO
    type Error = QueryEvaluationError;

    fn internal_quads_for_pattern(
        &self,
        _subject: Option<&Self::InternalTerm>,
        _predicate: Option<&Self::InternalTerm>,
        _object: Option<&Self::InternalTerm>,
        _graph_name: Option<Option<&Self::InternalTerm>>,
    ) -> impl Iterator<Item = Result<InternalQuad<Self::InternalTerm>, Self::Error>> + use<'a> {
        todo!(); // TODO
        #[allow(unreachable_code)]
        core::iter::empty()
    }

    fn internalize_term(&self, _term: Term) -> Result<Self::InternalTerm, Self::Error> {
        todo!() // TODO
    }

    fn externalize_term(&self, _term: Self::InternalTerm) -> Result<Term, Self::Error> {
        todo!() // TODO
    }
}
