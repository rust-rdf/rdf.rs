// This is free and unencumbered software released into the public domain.

extern crate alloc;

use alloc::{borrow::Cow, string::String};
use rdf_model::{HeapTerm, Term, TermKind};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BorshTerm(pub(crate) HeapTerm);

impl Term for BorshTerm {
    fn kind(&self) -> TermKind {
        self.0.kind()
    }

    fn as_str(&self) -> Cow<str> {
        self.0.as_str()
    }
}

impl From<&str> for BorshTerm {
    fn from(value: &str) -> Self {
        Self(HeapTerm::from(value))
    }
}

impl From<String> for BorshTerm {
    fn from(value: String) -> Self {
        Self(HeapTerm::from(value))
    }
}

impl From<HeapTerm> for BorshTerm {
    fn from(term: HeapTerm) -> Self {
        Self(term)
    }
}

impl From<&dyn Term> for BorshTerm {
    fn from(term: &dyn Term) -> Self {
        Self(HeapTerm::from(term))
    }
}

impl borsh::BorshSerialize for BorshTerm {
    fn serialize<W: borsh::io::Write>(&self, writer: &mut W) -> borsh::io::Result<()> {
        match &self.0 {
            HeapTerm::Iri(value) => {
                0x01u8.serialize(writer)?;
                value.serialize(writer)
            }
            HeapTerm::BNode(value) => {
                0x02u8.serialize(writer)?;
                value.serialize(writer)
            }
            HeapTerm::Literal(value) => {
                0x03u8.serialize(writer)?;
                value.serialize(writer)
            }
            HeapTerm::LiteralWithDatatype(value, datatype) => {
                0x04u8.serialize(writer)?;
                value.serialize(writer)?;
                datatype.serialize(writer)
            }
            HeapTerm::LiteralWithLanguage(value, language) => {
                0x05u8.serialize(writer)?;
                value.serialize(writer)?;
                language.serialize(writer)
            }
        }
    }
}

impl borsh::BorshDeserialize for BorshTerm {
    fn deserialize_reader<R: borsh::io::Read>(reader: &mut R) -> borsh::io::Result<Self> {
        Ok(BorshTerm(match u8::deserialize_reader(reader)? {
            0x01 => {
                let value = String::deserialize_reader(reader)?;
                HeapTerm::Iri(value)
            }
            0x02 => {
                let value = String::deserialize_reader(reader)?;
                HeapTerm::BNode(value)
            }
            0x03 => {
                let value = String::deserialize_reader(reader)?;
                HeapTerm::Literal(value)
            }
            0x04 => {
                let value = String::deserialize_reader(reader)?;
                let datatype = String::deserialize_reader(reader)?;
                HeapTerm::LiteralWithDatatype(value, datatype)
            }
            0x05 => {
                let value = String::deserialize_reader(reader)?;
                let language = String::deserialize_reader(reader)?;
                HeapTerm::LiteralWithLanguage(value, language)
            }
            _ => return Err(borsh::io::ErrorKind::InvalidData.into()),
        }))
    }
}
