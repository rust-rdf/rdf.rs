// This is free and unencumbered software released into the public domain.

use crate::MongoTripleId;
use mongodb::bson::{self, Bson, doc};
use rdf_model::{CowQuad, HeapQuad};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MongoTriple {
    pub _id: MongoTripleId,
    pub s: Bson,
    pub p: Bson,
    pub o: Bson,
}

impl MongoTriple {
    pub fn to_bson(&self) -> Bson {
        Bson::Document(self.to_doc())
    }

    pub fn to_doc(&self) -> bson::Document {
        doc! {
            "_id": self._id.to_bson(),
            "s": self.s.clone(),
            "p": self.p.clone(),
            "o": self.o.clone(),
        }
    }
}

impl From<MongoTriple> for Bson {
    fn from(input: MongoTriple) -> Self {
        input.to_bson()
    }
}

impl From<MongoTriple> for bson::Document {
    fn from(input: MongoTriple) -> Self {
        input.to_doc()
    }
}

impl From<&MongoTriple> for bson::Document {
    fn from(input: &MongoTriple) -> Self {
        input.to_doc()
    }
}

impl From<CowQuad<'_>> for MongoTriple {
    fn from(input: CowQuad<'_>) -> Self {
        let id = MongoTripleId::from(&input);
        let (s, p, o, _) = input.into_inner();
        Self {
            _id: id,
            s: s.into(),
            p: p.into(),
            o: o.into(),
        }
    }
}

impl From<&CowQuad<'_>> for MongoTriple {
    fn from(input: &CowQuad<'_>) -> Self {
        input.clone().into()
    }
}

impl From<HeapQuad> for MongoTriple {
    fn from(input: HeapQuad) -> Self {
        let id = MongoTripleId::from(&input);
        let (s, p, o, _) = input.into_inner();
        Self {
            _id: id,
            s: s.into(),
            p: p.into(),
            o: o.into(),
        }
    }
}

impl From<&HeapQuad> for MongoTriple {
    fn from(input: &HeapQuad) -> Self {
        input.clone().into()
    }
}
