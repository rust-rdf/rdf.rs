// This is free and unencumbered software released into the public domain.

extern crate alloc;

use alloc::{borrow::Cow, format, string::String};

const RDF_LANG_STRING: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString";
const RDF_DIR_LANG_STRING: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#dirLangString";

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
    /// See: https://www.w3.org/TR/rdf12-concepts/#section-Graph-Literal
    RdfLangString,

    /// See: https://www.w3.org/TR/rdf12-concepts/#section-Graph-Literal
    RdfDirLangString,

    /// See: https://www.w3.org/TR/xmlschema-2/
    Xsd(xsd::Type),

    /// See: https://www.w3.org/TR/rdf12-concepts/#section-Datatypes
    Other(Cow<'static, str>),
}

impl Datatype {
    pub fn from_iri(iri: impl AsRef<str>) -> Self {
        Self::from(iri.as_ref())
    }

    pub fn curie(&self) -> Option<&str> {
        Some(match self {
            Datatype::RdfLangString => "rdf:langString",
            Datatype::RdfDirLangString => "rdf:dirLangString",
            Datatype::Xsd(xsd) => xsd.curie(),
            Datatype::Other(_) => return None, // not a CURIE
        })
    }

    pub fn iri_string(&self) -> Cow<'_, str> {
        match self {
            Datatype::RdfLangString => Cow::Borrowed(RDF_LANG_STRING),
            Datatype::RdfDirLangString => Cow::Borrowed(RDF_DIR_LANG_STRING),
            Datatype::Xsd(xsd) => xsd.iri_string(),
            Datatype::Other(other) => other.clone(),
        }
    }
}

impl core::fmt::Display for Datatype {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if let Some(curie) = self.curie() {
            f.write_str(curie)
        } else {
            f.write_str(self.iri_string().as_ref())
        }
    }
}

impl From<&str> for Datatype {
    fn from(input: &str) -> Self {
        if input == RDF_LANG_STRING {
            Datatype::RdfLangString
        } else if input == RDF_DIR_LANG_STRING {
            Datatype::RdfDirLangString
        } else if input.starts_with(xsd::BASE_URI) {
            Datatype::Xsd(xsd::Type::from(&input[xsd::BASE_URI.len()..]))
        } else {
            Datatype::Other(Cow::Owned(input.into()))
        }
    }
}

impl From<Cow<'_, str>> for Datatype {
    fn from(input: Cow<'_, str>) -> Self {
        Self::from(input.as_ref())
    }
}

impl From<String> for Datatype {
    fn from(input: String) -> Self {
        Self::from(input.as_str())
    }
}

impl From<&String> for Datatype {
    fn from(input: &String) -> Self {
        Self::from(input.as_str())
    }
}

impl From<xsd::Type> for Datatype {
    fn from(input: xsd::Type) -> Self {
        Self::Xsd(input)
    }
}

impl From<&xsd::Type> for Datatype {
    fn from(input: &xsd::Type) -> Self {
        Self::Xsd(input.clone())
    }
}
