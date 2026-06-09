// This is free and unencumbered software released into the public domain.

use crate::{BaseDirection, CowTerm, Datatype, Term, TermKind};
use alloc::{
    borrow::Cow,
    string::{String, ToString},
};
use xsd::{DecimalValue, ParseError, PrimitiveValue, Value, primitive::Boolean};

type Language = String; // TODO

/// A heap-allocated term.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
// #[cfg_attr(
//     feature = "borsh",
//     derive(borsh::BorshSerialize, borsh::BorshDeserialize)
// )]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HeapTerm {
    /// An IRI or IRI reference term.
    Iri(String),

    /// A blank node (aka bnode) term.
    BNode(String),

    /// A plain string term.
    /// The term's datatype is `xsd:string`.
    String(String),

    /// A language-tagged string term.
    /// The term's datatype is either `rdf:langString` or `rdf:dirLangString`.
    TaggedString(String, Language, Option<BaseDirection>),

    /// A typed value term.
    TypedValue(Value),

    /// A typed literal term that could not be instantiated as a typed value.
    /// The term is either an ill-typed literal and/or an unsupported datatype.
    TypedLiteral(String, Datatype),
}

impl HeapTerm {
    pub fn iri(value: impl Into<String>) -> Self {
        Self::Iri(value.into())
    }

    pub fn bnode(id: impl Into<String>) -> Self {
        Self::BNode(id.into())
    }

    pub fn string(value: impl Into<String>) -> Self {
        Self::String(value.into())
    }

    pub fn tagged_string(value: impl Into<String>, tag: impl Into<Language>) -> Self {
        Self::TaggedString(value.into(), tag.into(), None)
    }

    pub fn typed_value(value: impl Into<Value>) -> Self {
        Self::TypedValue(value.into())
    }

    pub fn typed_literal(literal: impl Into<String>, datatype: impl Into<Datatype>) -> Self {
        Self::TypedLiteral(literal.into(), datatype.into())
    }

    pub fn kind(&self) -> TermKind {
        match self {
            Self::Iri(_) => TermKind::Iri,
            Self::BNode(_) => TermKind::BNode,
            Self::String(_)
            | Self::TaggedString(_, _, _)
            | Self::TypedValue(_)
            | Self::TypedLiteral(_, _) => TermKind::Literal,
        }
    }

    pub fn value_str(&self) -> Cow<'_, str> {
        Cow::Borrowed(match self {
            Self::Iri(s) => s.as_str(),
            Self::BNode(s) => s.as_str(),
            Self::String(s) | Self::TaggedString(s, _, _) | Self::TypedLiteral(s, _) => s.as_str(),
            Self::TypedValue(Value::Primitive(PrimitiveValue::Boolean(Boolean::FALSE))) => "false",
            Self::TypedValue(Value::Primitive(PrimitiveValue::Boolean(Boolean::TRUE))) => "true",
            Self::TypedValue(Value::Primitive(
                PrimitiveValue::String(s) | PrimitiveValue::AnyUri(s),
            )) => s.as_str(),
            Self::TypedValue(v) => return Cow::Owned(v.to_string()),
        })
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Option<serde_json::Value> {
        Some(self.clone().into_json())
    }

    #[cfg(feature = "serde")]
    pub fn into_json(self) -> serde_json::Value {
        use serde_json::{Number as JsonNumber, Value as JsonValue, json};
        match self {
            Self::Iri(str) => JsonValue::String(str),
            Self::BNode(id) => JsonValue::String(id),
            Self::String(val) => json!({ "@value": val }),
            Self::TaggedString(val, lang, _) => json!({ "@value": val, "@language": lang }), // TODO: direction
            Self::TypedLiteral(val, r#type) => {
                json!({ "@value": val, "@type": r#type.to_string() })
            },
            Self::TypedValue(Value::Primitive(val)) => match val {
                PrimitiveValue::String(val) => json!({ "@value": val }),
                PrimitiveValue::Boolean(val) => val.into_json(),
                PrimitiveValue::Double(val) => val.into_json(),
                val => {
                    let r#type = val.r#type();
                    json!({ "@value": val.into_json(), "@type": r#type.curie() })
                },
            },
            Self::TypedValue(Value::Decimal(val)) => match val {
                DecimalValue::Integer(val) => json!({ "@value": val, "@type": "xsd:integer" }),
                val => {
                    let r#type = val.r#type();
                    json!({ "@value": val.into_json(), "@type": r#type.curie() })
                },
            },
        }
    }

    #[cfg(feature = "blake3")]
    #[deprecated(since = "0.3.5", note = "Use `into::<rdf_hash::TermHash>()`")]
    pub fn to_b3(&self) -> crate::TermHash {
        blake3::hash(self.value_str().as_ref().as_bytes()) // TODO: N-Triples
    }
}

impl Term for HeapTerm {
    fn kind(&self) -> TermKind {
        self.kind()
    }

    fn value_str(&self) -> Cow<'_, str> {
        self.value_str()
    }
}

impl Term for &HeapTerm {
    fn kind(&self) -> TermKind {
        (*self).kind()
    }

    fn value_str(&self) -> Cow<'_, str> {
        (*self).value_str()
    }
}

impl From<&dyn Term> for HeapTerm {
    fn from(term: &dyn Term) -> Self {
        match term.kind() {
            TermKind::Iri => Self::iri(term.value_str()),
            TermKind::BNode => Self::bnode(term.value_str()),
            TermKind::Literal => Self::string(term.value_str()), // FIXME
        }
    }
}

impl From<CowTerm<'_>> for HeapTerm {
    fn from(input: CowTerm<'_>) -> Self {
        Self::from(&input)
    }
}

impl From<&CowTerm<'_>> for HeapTerm {
    fn from(input: &CowTerm<'_>) -> Self {
        match input {
            CowTerm::Iri(s) => HeapTerm::Iri(s.to_string()),
            CowTerm::BNode(s) => HeapTerm::BNode(s.to_string()),
            CowTerm::String(s) => HeapTerm::String(s.to_string()),
            CowTerm::TaggedString(s, lang, dir) => {
                HeapTerm::TaggedString(s.to_string(), lang.clone(), dir.clone())
            },
            CowTerm::TypedValue(v) => HeapTerm::TypedValue(v.clone()),
            CowTerm::TypedLiteral(s, dt) => HeapTerm::TypedLiteral(s.to_string(), dt.clone()),
        }
    }
}

impl From<&str> for HeapTerm {
    fn from(value: &str) -> Self {
        Self::string(value)
    }
}

impl From<String> for HeapTerm {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&String> for HeapTerm {
    fn from(value: &String) -> Self {
        Self::String(value.clone())
    }
}

impl From<Boolean> for HeapTerm {
    fn from(value: Boolean) -> Self {
        Self::TypedValue(value.into())
    }
}

impl From<(String, Language)> for HeapTerm {
    fn from((literal, language): (String, Language)) -> Self {
        Self::TaggedString(literal, language, None)
    }
}

impl From<(String, Datatype)> for HeapTerm {
    fn from((literal, datatype): (String, Datatype)) -> Self {
        let result = match &datatype {
            Datatype::Xsd(t) => xsd::parse(&literal, t),
            _ => Err(ParseError),
        };
        match result {
            Ok(value) => Self::TypedValue(value),
            Err(_) => Self::TypedLiteral(literal, datatype),
        }
    }
}

impl From<(String, Option<Datatype>, Option<Language>)> for HeapTerm {
    fn from((literal, datatype, language): (String, Option<Datatype>, Option<Language>)) -> Self {
        match (language, datatype) {
            (None, None) => Self::string(literal),
            (None, Some(datatype)) => Self::from((literal, datatype)),
            (Some(language), None) => Self::tagged_string(literal, language),
            (Some(language), Some(_)) => Self::tagged_string(literal, language), // TODO
        }
    }
}

#[cfg(feature = "serde")]
impl TryFrom<serde_json::Value> for HeapTerm {
    type Error = ();

    fn try_from(input: serde_json::Value) -> Result<Self, Self::Error> {
        use serde_json::Value;
        Ok(match input {
            Value::Null => return Err(()), // not supported
            Value::Bool(b) => HeapTerm::TypedValue(b.into()),
            Value::Number(n) => HeapTerm::TypedValue(n.as_f64().unwrap().into()), // FIXME
            Value::String(s) if s.starts_with("_:") => HeapTerm::BNode(s.clone()),
            Value::String(s) => HeapTerm::Iri(s.clone()),
            Value::Array(_) => return Err(()), // not supported
            Value::Object(mut input) => {
                let value = input.remove("@value").ok_or(())?;
                let r#type = input
                    .remove("@type")
                    .and_then(|s| s.as_str().map(|s| s.parse::<Datatype>().ok()))
                    .flatten();
                let language = input
                    .remove("@language")
                    .and_then(|s| s.as_str().map(|s| s.to_string()));
                match (r#type, language) {
                    (None, None) => HeapTerm::String(value.to_string()),
                    (None, Some(lang)) => {
                        HeapTerm::TaggedString(value.to_string(), lang.into(), None)
                    },
                    (Some(r#type), None) => HeapTerm::TypedLiteral(value.to_string(), r#type),
                    (Some(_), Some(_)) => return Err(()), // invalid literal
                }
            },
        })
    }
}

#[cfg(feature = "serde")]
impl TryFrom<&serde_json::Value> for HeapTerm {
    type Error = ();

    fn try_from(input: &serde_json::Value) -> Result<Self, Self::Error> {
        input.clone().try_into()
    }
}
