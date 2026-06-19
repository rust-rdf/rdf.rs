// This is free and unencumbered software released into the public domain.

use crate::{CowTerm, HeapTerm, interop::OxrdfTerm};
use alloc::{
    borrow::Cow,
    string::{String, ToString},
};

impl From<oxrdf::Term> for OxrdfTerm {
    fn from(input: oxrdf::Term) -> Self {
        Self(input)
    }
}

impl From<oxrdf::Term> for HeapTerm {
    fn from(input: oxrdf::Term) -> Self {
        use oxrdf::{Literal, Term};
        match input {
            Term::NamedNode(iri) => Self::Iri(iri.into_string()),
            Term::BlankNode(id) => Self::BNode(id.into_string()),
            Term::Literal(lit) => match lit.destruct() {
                (val, None, None, None) => Self::string(val),
                (val, None, Some(lang), None) => Self::tagged_string(val, lang),
                (val, None, Some(lang), Some(dir)) => Self::tagged_string_with_dir(val, lang, dir),
                (lit, Some(typ), None, None) => Self::typed_literal(lit, typ), // FIXME: TypedValue
                (_, None, None, Some(_)) => unreachable!(),
                (_, Some(_), None, Some(_)) => unreachable!(),
                (_, Some(_), Some(_), None) => unreachable!(),
                (_, Some(_), Some(_), Some(_)) => unreachable!(),
            },
            Term::Triple(_) => todo!("RDF-star support not implemented yet"), // TODO
        }
    }
}

impl<'a> From<oxrdf::Term> for CowTerm<'_> {
    fn from(input: oxrdf::Term) -> Self {
        use oxrdf::Term;
        match input {
            Term::NamedNode(iri) => CowTerm::Iri(iri.into_string().into()),
            Term::BlankNode(id) => CowTerm::BNode(id.into_string().into()),
            Term::Literal(lit) => match lit.destruct() {
                (val, None, None, None) => Self::string(val),
                (val, None, Some(lang), None) => Self::tagged_string(val, lang),
                (val, None, Some(lang), Some(dir)) => Self::tagged_string_with_dir(val, lang, dir),
                (lit, Some(typ), None, None) => Self::typed_literal(lit, typ), // FIXME: TypedValue
                (_, None, None, Some(_)) => unreachable!(),
                (_, Some(_), None, Some(_)) => unreachable!(),
                (_, Some(_), Some(_), None) => unreachable!(),
                (_, Some(_), Some(_), Some(_)) => unreachable!(),
            },
            Term::Triple(_) => todo!("RDF-star support not implemented yet"), // TODO
        }
    }
}
