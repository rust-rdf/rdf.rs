// This is free and unencumbered software released into the public domain.

extern crate alloc;

use crate::{Term, TermKind};
use alloc::borrow::Cow;
use sophia::api::term::SimpleTerm;

pub struct SophiaTerm<'a> {
    inner: SimpleTerm<'a>,
}

impl<'a> Term for SophiaTerm<'a> {
    fn kind(&self) -> TermKind {
        use sophia::api::term::Term;
        self.inner.kind().into()
    }

    fn as_str(&self) -> Cow<str> {
        match &self.inner {
            SimpleTerm::Iri(iri) => Cow::Borrowed(iri.as_str()),
            SimpleTerm::BlankNode(id) => Cow::Borrowed(id.as_str()),
            SimpleTerm::LiteralDatatype(value, _iri) => Cow::Borrowed(value.as_ref()), // TODO
            SimpleTerm::LiteralLanguage(value, _lang) => Cow::Borrowed(value.as_ref()), // TODO
            SimpleTerm::Triple(_) => todo!(),                                          // TODO
            SimpleTerm::Variable(_) => todo!(),                                        // TODO
        }
    }
}
