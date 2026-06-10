// This is free and unencumbered software released into the public domain.

use mongodb::bson::{self, Binary, Bson, doc, spec::BinarySubtype};
use rdf_hash::TripleHash;
use rdf_model::{CowQuad, HeapQuad};

/// A triple ID used to identify a triple in MongoDB.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MongoTripleId(TripleHash);

impl MongoTripleId {
    pub fn to_bson(&self) -> Bson {
        Bson::Binary(Binary {
            subtype: BinarySubtype::Generic,
            bytes: self.0.to_vec(),
        })
    }
}

impl core::fmt::Display for MongoTripleId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<MongoTripleId> for bson::Document {
    fn from(input: MongoTripleId) -> Self {
        doc! { "_id": input.to_bson() }
    }
}

impl From<&CowQuad<'_>> for MongoTripleId {
    fn from(input: &CowQuad<'_>) -> Self {
        Self(input.into())
    }
}

impl From<&HeapQuad> for MongoTripleId {
    fn from(input: &HeapQuad) -> Self {
        Self(input.into())
    }
}
