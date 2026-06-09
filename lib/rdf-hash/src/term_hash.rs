// This is free and unencumbered software released into the public domain.

use blake3::Hash;

/// The length of a term hash in bytes.
pub const TERM_HASH_LEN: usize = blake3::OUT_LEN;

/// The all-zero bytes value of a term hash.
pub const TERM_HASH_ZERO: [u8; TERM_HASH_LEN] = [0u8; TERM_HASH_LEN];

/// A cryptographically-secure hash of a term.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TermHash(Hash);

impl TermHash {
    /// The all-zero term hash.
    pub const ZERO: Self = Self::from_bytes(TERM_HASH_ZERO);

    pub const fn from_bytes(input: [u8; TERM_HASH_LEN]) -> Self {
        Self(Hash::from_bytes(input))
    }

    pub fn as_bytes(&self) -> &[u8; TERM_HASH_LEN] {
        self.0.as_bytes()
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl Default for TermHash {
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<[u8; TERM_HASH_LEN]> for TermHash {
    fn from(input: [u8; TERM_HASH_LEN]) -> Self {
        Self::from_bytes(input)
    }
}

impl From<&[u8; TERM_HASH_LEN]> for TermHash {
    fn from(input: &[u8; TERM_HASH_LEN]) -> Self {
        Self::from_bytes(*input)
    }
}

impl From<TermHash> for [u8; TERM_HASH_LEN] {
    fn from(input: TermHash) -> Self {
        input.0.into()
    }
}

impl From<Hash> for TermHash {
    fn from(input: Hash) -> Self {
        Self(input)
    }
}

impl From<&Hash> for TermHash {
    fn from(input: &Hash) -> Self {
        Self(input.clone())
    }
}

impl From<TermHash> for Hash {
    fn from(input: TermHash) -> Self {
        input.0
    }
}

impl From<&TermHash> for Hash {
    fn from(input: &TermHash) -> Self {
        input.0.clone()
    }
}
