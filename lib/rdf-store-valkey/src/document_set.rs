// This is free and unencumbered software released into the public domain.

use super::ValkeyDocument;
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
};
use itertools::Itertools;
use rdf_model::{HeapQuadSet, Statement};

#[allow(unused)]
#[derive(Debug, Default, Eq, PartialEq)]
pub struct ValkeyDocumentSet(pub(crate) BTreeMap<String, ValkeyDocument>);

impl From<HeapQuadSet> for ValkeyDocumentSet {
    fn from(input: HeapQuadSet) -> Self {
        let mut docs = BTreeMap::<String, ValkeyDocument>::default();
        for (subject, quads) in input
            .into_iter()
            .chunk_by(|quad| quad.subject().clone())
            .into_iter()
        {
            let mut doc = ValkeyDocument::default();
            for quad in quads {
                let (_, p, o, _g) = quad.into_inner();
                let p = p.value_str();
                let o = o.into_json();
                let os = doc.0.entry(p.to_string()).or_default();
                if !os.contains(&o) {
                    os.push(o);
                }
            }
            docs.insert(subject.value_str().to_string(), doc);
        }
        Self(docs)
    }
}

impl From<ValkeyDocumentSet> for serde_json::Value {
    fn from(_input: ValkeyDocumentSet) -> Self {
        todo!() // TODO
    }
}
