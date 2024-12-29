// This is free and unencumbered software released into the public domain.

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TermKind {
    Iri,
    BNode,
    Literal,
}

#[cfg(feature = "sophia")]
impl From<sophia::api::term::TermKind> for TermKind {
    fn from(kind: sophia::api::term::TermKind) -> Self {
        use sophia::api::term::TermKind::*;
        match kind {
            Iri => Self::Iri,
            BlankNode => Self::BNode,
            Literal => Self::Literal,
            _ => todo!(), // TODO
        }
    }
}
