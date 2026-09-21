// This is free and unencumbered software released into the public domain.

/// The kind of an RDF term or the distinguished default-graph marker.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TermKind {
    /// An IRI or IRI reference.
    Iri,
    /// A blank node.
    BNode,
    /// A literal, including its datatype, language, and direction metadata.
    Literal,
    /// The default-graph singleton, not an ordinary RDF term.
    DefaultGraph,
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
