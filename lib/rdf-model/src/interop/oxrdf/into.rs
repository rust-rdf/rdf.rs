// This is free and unencumbered software released into the public domain.

use std::string::ToString;

use crate::{CowTerm, HeapTerm, Term, interop::OxrdfTerm};

/// Provides `Into<oxrdf::Term>` for `OxrdfTerm`.
impl From<OxrdfTerm> for oxrdf::Term {
    fn from(input: OxrdfTerm) -> Self {
        input.0
    }
}

/// Provides `Into<oxrdf::Term>` for `HeapTerm`.
impl From<HeapTerm> for oxrdf::Term {
    fn from(input: HeapTerm) -> Self {
        use oxrdf::{BaseDirection, BlankNode, Literal, NamedNode, Term};
        match input {
            HeapTerm::Iri(iri) => Term::NamedNode(NamedNode::new_unchecked(iri)),
            HeapTerm::BNode(id) => Term::BlankNode(BlankNode::new_unchecked(id)),
            HeapTerm::String(val) => Term::Literal(Literal::new_simple_literal(val)),
            HeapTerm::TaggedString(val, lang, None) => {
                Term::Literal(Literal::new_language_tagged_literal_unchecked(val, lang))
            },
            HeapTerm::TaggedString(val, lang, Some(dir)) => {
                Term::Literal(Literal::new_directional_language_tagged_literal_unchecked(
                    val,
                    lang,
                    BaseDirection::from(dir),
                ))
            },
            HeapTerm::TypedLiteral(lit, typ) => Term::Literal(Literal::new_typed_literal(
                lit,
                NamedNode::new_unchecked(typ.iri_string()),
            )),
            HeapTerm::TypedValue(val) => Term::Literal(Literal::new_typed_literal(
                val.to_string(),
                NamedNode::new_unchecked(val.r#type().iri_string()),
            )),
        }
    }
}

/// Provides `Into<oxrdf::Term>` for `CowTerm`.
impl From<CowTerm<'_>> for oxrdf::Term {
    fn from(input: CowTerm<'_>) -> Self {
        use oxrdf::{BaseDirection, BlankNode, Literal, NamedNode, Term};
        match input {
            CowTerm::Iri(iri) => Term::NamedNode(NamedNode::new_unchecked(iri)),
            CowTerm::BNode(id) => Term::BlankNode(BlankNode::new_unchecked(id)),
            CowTerm::String(val) => Term::Literal(Literal::new_simple_literal(val)),
            CowTerm::TaggedString(val, lang, None) => {
                Term::Literal(Literal::new_language_tagged_literal_unchecked(val, lang))
            },
            CowTerm::TaggedString(val, lang, Some(dir)) => {
                Term::Literal(Literal::new_directional_language_tagged_literal_unchecked(
                    val,
                    lang,
                    BaseDirection::from(dir),
                ))
            },
            CowTerm::TypedLiteral(lit, typ) => Term::Literal(Literal::new_typed_literal(
                lit,
                NamedNode::new_unchecked(typ.iri_string()),
            )),
            CowTerm::TypedValue(val) => Term::Literal(Literal::new_typed_literal(
                val.to_string(),
                NamedNode::new_unchecked(val.r#type().iri_string()),
            )),
        }
    }
}
