// This is free and unencumbered software released into the public domain.

use crate::{Term, TermKind};
use sophia::api::term::SimpleTerm;

pub struct SophiaTerm<'a> {
    inner: SimpleTerm<'a>,
}

impl<'a> Term for SophiaTerm<'a> {
    fn kind(&self) -> TermKind {
        use sophia::api::term::Term;
        self.inner.kind().into()
    }

    #[cfg(feature = "alloc")]
    fn value_str(&self) -> alloc::borrow::Cow<'_, str> {
        use alloc::borrow::Cow;
        match &self.inner {
            SimpleTerm::Iri(iri) => Cow::Borrowed(iri.as_str()),
            SimpleTerm::BlankNode(id) => Cow::Borrowed(id.as_str()),
            SimpleTerm::LiteralDatatype(value, _iri) => Cow::Borrowed(value.as_ref()), // TODO
            SimpleTerm::LiteralLanguage(value, _lang, _) => Cow::Borrowed(value.as_ref()), // TODO
            SimpleTerm::Triple(_) => todo!(),                                          // TODO
            SimpleTerm::Variable(_) => todo!(),                                        // TODO
        }
    }
}
