// This is free and unencumbered software released into the public domain.

use alloc::string::{String, ToString};
use fred::types::Key;
use serde_json::Value;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ValkeyTripleKey(pub(crate) String);

impl From<String> for ValkeyTripleKey {
    fn from(input: String) -> Self {
        Self(input)
    }
}

impl From<&String> for ValkeyTripleKey {
    fn from(input: &String) -> Self {
        Self(input.clone())
    }
}

impl From<ValkeyTripleKey> for String {
    fn from(input: ValkeyTripleKey) -> Self {
        input.0
    }
}

impl From<&ValkeyTripleKey> for String {
    fn from(input: &ValkeyTripleKey) -> Self {
        input.0.clone()
    }
}

impl From<ValkeyTripleKey> for Key {
    fn from(input: ValkeyTripleKey) -> Self {
        Key::from(input.to_string())
    }
}

impl From<ValkeyTripleKey> for Value {
    fn from(input: ValkeyTripleKey) -> Self {
        Value::String(input.into())
    }
}

impl From<&ValkeyTripleKey> for Value {
    fn from(input: &ValkeyTripleKey) -> Self {
        Value::String(input.into())
    }
}

impl core::fmt::Display for ValkeyTripleKey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_fmt(format_args!("rdf:j:{}", self.0))
    }
}
