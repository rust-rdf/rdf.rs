// This is free and unencumbered software released into the public domain.

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use arrayvec::ArrayString;
use core::{hash::Hash, str::FromStr};
use rdf_model::{TERM_HASH_LEN, TermHash};
use serde_json::Value;

/// A triple ID used to identify a triple in Valkey.
#[derive(Clone, Default, Eq, Hash, PartialEq)]
pub struct ValkeyTripleId(
    pub(crate) Option<TermHash>,
    pub(crate) Option<TermHash>,
    pub(crate) Option<TermHash>,
);

impl From<(TermHash, TermHash, TermHash)> for ValkeyTripleId {
    fn from(input: (TermHash, TermHash, TermHash)) -> Self {
        Self(Some(input.0), Some(input.1), Some(input.2))
    }
}

impl FromStr for ValkeyTripleId {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.split(':').collect();
        if parts.len() != 3 {
            return Err(());
        }
        let mut s = hex::decode(parts[0]).map_err(|_| ())?;
        let mut p = hex::decode(parts[1]).map_err(|_| ())?;
        let mut o = hex::decode(parts[2]).map_err(|_| ())?;
        s.resize(TERM_HASH_LEN, 0u8);
        p.resize(TERM_HASH_LEN, 0u8);
        o.resize(TERM_HASH_LEN, 0u8);
        let s = TermHash::from_slice(&s).map_err(|_| ())?;
        let p = TermHash::from_slice(&p).map_err(|_| ())?;
        let o = TermHash::from_slice(&o).map_err(|_| ())?;
        Ok(Self(Some(s), Some(p), Some(o)))
    }
}

impl From<ValkeyTripleId> for String {
    fn from(input: ValkeyTripleId) -> Self {
        input.to_string()
    }
}

impl From<&ValkeyTripleId> for String {
    fn from(input: &ValkeyTripleId) -> Self {
        input.to_string()
    }
}

impl From<ValkeyTripleId> for fred::types::Key {
    fn from(input: ValkeyTripleId) -> Self {
        fred::types::Key::from(input.to_string())
    }
}

impl From<ValkeyTripleId> for fred::types::Value {
    fn from(input: ValkeyTripleId) -> Self {
        fred::types::Value::String(input.to_string().into())
    }
}

impl From<&ValkeyTripleId> for fred::types::Value {
    fn from(input: &ValkeyTripleId) -> Self {
        fred::types::Value::String(input.to_string().into())
    }
}

impl From<ValkeyTripleId> for Value {
    fn from(input: ValkeyTripleId) -> Self {
        Value::String(input.to_string())
    }
}

impl From<&ValkeyTripleId> for Value {
    fn from(input: &ValkeyTripleId) -> Self {
        Value::String(input.to_string())
    }
}

impl core::fmt::Display for ValkeyTripleId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let default = ArrayString::from("*").unwrap();
        f.write_fmt(format_args!(
            "{:.8}:{:.8}:{:.8}",
            self.0.map(|h| h.to_hex()).unwrap_or(default),
            self.1.map(|h| h.to_hex()).unwrap_or(default),
            self.2.map(|h| h.to_hex()).unwrap_or(default)
        ))
    }
}

impl core::fmt::Debug for ValkeyTripleId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}
