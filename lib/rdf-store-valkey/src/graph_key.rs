// This is free and unencumbered software released into the public domain.

use crate::ValkeyTerm;
use alloc::{
    borrow::Cow,
    string::{String, ToString},
};
use fred::types::Key;
use serde_json::Value;

/// A graph key for fetching a graph from Valkey.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ValkeyGraphKey(pub(crate) String);

impl Default for ValkeyGraphKey {
    fn default() -> Self {
        Self("default".to_string())
    }
}

impl<T> From<&T> for ValkeyGraphKey
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<&Option<ValkeyTerm>> for ValkeyGraphKey {
    fn from(input: &Option<ValkeyTerm>) -> Self {
        match input {
            None => Self::default(),
            Some(term) => Self(term.0.as_str().unwrap().to_string()),
        }
    }
}

impl From<&Cow<'_, str>> for ValkeyGraphKey {
    fn from(input: &Cow<'_, str>) -> Self {
        Self(input.to_string())
    }
}

impl From<String> for ValkeyGraphKey {
    fn from(input: String) -> Self {
        Self(input)
    }
}

impl From<ValkeyGraphKey> for String {
    fn from(input: ValkeyGraphKey) -> Self {
        input.0
    }
}

impl From<&ValkeyGraphKey> for String {
    fn from(input: &ValkeyGraphKey) -> Self {
        input.0.clone()
    }
}

impl From<ValkeyGraphKey> for Key {
    fn from(input: ValkeyGraphKey) -> Self {
        Key::from(input.to_string())
    }
}

impl From<&ValkeyGraphKey> for Key {
    fn from(input: &ValkeyGraphKey) -> Self {
        Key::from(input.to_string())
    }
}

impl From<ValkeyGraphKey> for Value {
    fn from(input: ValkeyGraphKey) -> Self {
        Value::String(input.into())
    }
}

impl From<&ValkeyGraphKey> for Value {
    fn from(input: &ValkeyGraphKey) -> Self {
        Value::String(input.into())
    }
}

impl core::fmt::Display for ValkeyGraphKey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_fmt(format_args!("rdf:g:{}", self.0))
    }
}
