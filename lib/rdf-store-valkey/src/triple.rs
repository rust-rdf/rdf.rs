// This is free and unencumbered software released into the public domain.

use alloc::{
    format,
    string::{String, ToString},
};
use rdf_model::HeapTriple;
use serde_json::{Map, Value};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ValkeyTriple {
    pub(crate) id: String,
    pub(crate) s: Value,
    pub(crate) p: Value,
    pub(crate) o: Value,
}

impl ValkeyTriple {
    pub fn id(&self) -> &String {
        &self.id
    }
}

impl From<HeapTriple> for ValkeyTriple {
    fn from(input: HeapTriple) -> Self {
        let (s, p, o) = input.into_inner();
        let id = format!(
            "{}:{}:{}",
            s.to_b3().to_hex()[0..16].to_lowercase(),
            p.to_b3().to_hex()[0..16].to_lowercase(),
            o.to_b3().to_hex()[0..16].to_lowercase()
        );
        Self {
            id,
            s: s.into_json(),
            p: p.into_json(),
            o: o.into_json(),
        }
    }
}

impl From<ValkeyTriple> for Value {
    fn from(input: ValkeyTriple) -> Self {
        let mut output = Map::new();
        output.insert("s".to_string(), input.s);
        output.insert("p".to_string(), input.p);
        output.insert("o".to_string(), input.o);
        Value::Object(output)
    }
}
