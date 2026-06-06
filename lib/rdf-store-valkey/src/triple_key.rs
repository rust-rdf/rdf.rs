// This is free and unencumbered software released into the public domain.

use crate::ValkeyTripleId;
use alloc::string::{String, ToString};
use fred::types::Key;
use serde_json::Value;

/// A triple key for fetching a triple from Valkey.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct ValkeyTripleKey(pub(crate) ValkeyTripleId);

impl From<ValkeyTripleId> for ValkeyTripleKey {
    fn from(input: ValkeyTripleId) -> Self {
        Self(input)
    }
}

impl From<&ValkeyTripleId> for ValkeyTripleKey {
    fn from(input: &ValkeyTripleId) -> Self {
        Self(input.clone())
    }
}

impl From<ValkeyTripleKey> for String {
    fn from(input: ValkeyTripleKey) -> Self {
        input.to_string()
    }
}

impl From<&ValkeyTripleKey> for String {
    fn from(input: &ValkeyTripleKey) -> Self {
        input.to_string()
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
