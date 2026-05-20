// This is free and unencumbered software released into the public domain.

extern crate alloc;

use alloc::{borrow::Cow, format, string::String};

const RDF_LANG_STRING: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString";

/// A datatype.
///
/// See: https://www.w3.org/TR/rdf12-concepts/#section-Datatypes
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Datatype {
    /// See: https://www.w3.org/TR/rdf12-concepts/#section-Datatypes
    RdfLangString,

    /// See: https://www.w3.org/TR/xmlschema-2/
    Xsd(xsd::Type),

    /// See: https://www.w3.org/TR/rdf12-concepts/#section-Datatypes
    Other(Cow<'static, str>),
}

impl Datatype {
    pub fn curie(&self) -> &str {
        match self {
            Datatype::RdfLangString => "rdf:langString",
            Datatype::Xsd(xsd) => xsd.curie(),
            Datatype::Other(other) => other.as_ref(),
        }
    }

    pub fn iri_string(&self) -> Cow<'_, str> {
        match self {
            Datatype::RdfLangString => Cow::Borrowed(RDF_LANG_STRING),
            Datatype::Xsd(xsd) => xsd.iri_string(),
            Datatype::Other(other) => other.clone(),
        }
    }
}
