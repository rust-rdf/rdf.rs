// This is free and unencumbered software released into the public domain.

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use rdf_model::HeapQuadSet;
use serde_json::{Map, Value};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ValkeyDocument(pub(crate) BTreeMap<String, Vec<Value>>);

impl From<HeapQuadSet> for ValkeyDocument {
    fn from(input: HeapQuadSet) -> Self {
        let mut doc = BTreeMap::<String, Vec<Value>>::default();
        for quad in input {
            let (_, p, o, _g) = quad.into_inner();
            let p = p.value_str();
            let o = o.into_json();
            let os = doc.entry(p.to_string()).or_default();
            if !os.contains(&o) {
                os.push(o);
            }
        }
        Self(doc)
    }
}

impl From<ValkeyDocument> for serde_json::Value {
    fn from(input: ValkeyDocument) -> Self {
        let mut output: Map<String, Value> = Map::new();
        for (k, v) in input.0 {
            output.insert(k, v.into());
        }
        Value::Object(output)
    }
}
