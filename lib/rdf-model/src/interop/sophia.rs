// This is free and unencumbered software released into the public domain.

use crate::{DEFAULT_GRAPH_URN, DefaultGraph, Term, TermKind};
use sophia::api::term::SimpleTerm;

/// A Sophia term or the distinct default-graph singleton.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SophiaTerm<'a> {
    inner: Option<SimpleTerm<'a>>,
}

impl<'a> Term for SophiaTerm<'a> {
    fn kind(&self) -> TermKind {
        use sophia::api::term::Term;
        self.inner
            .as_ref()
            .map_or(TermKind::DefaultGraph, |term| term.kind().into())
    }

    #[cfg(feature = "alloc")]
    fn value_str(&self) -> alloc::borrow::Cow<'_, str> {
        use alloc::borrow::Cow;
        let Some(inner) = &self.inner else {
            return Cow::Borrowed(DEFAULT_GRAPH_URN);
        };
        match inner {
            SimpleTerm::Iri(iri) => Cow::Borrowed(iri.as_str()),
            SimpleTerm::BlankNode(id) => Cow::Borrowed(id.as_str()),
            SimpleTerm::LiteralDatatype(value, _iri) => Cow::Borrowed(value.as_ref()), // TODO
            SimpleTerm::LiteralLanguage(value, _lang, _) => Cow::Borrowed(value.as_ref()), // TODO
            SimpleTerm::Triple(_) => todo!(),                                          // TODO
            SimpleTerm::Variable(_) => todo!(),                                        // TODO
        }
    }
}

impl From<DefaultGraph> for SophiaTerm<'_> {
    fn from(_: DefaultGraph) -> Self {
        Self { inner: None }
    }
}

impl<'a> From<SimpleTerm<'a>> for SophiaTerm<'a> {
    fn from(inner: SimpleTerm<'a>) -> Self {
        Self { inner: Some(inner) }
    }
}
