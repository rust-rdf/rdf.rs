// This is free and unencumbered software released into the public domain.

use crate::TermHash;
use blake3::{Hash, Hasher};
use rdf_model::{CowTerm, HeapTerm};

/// The length of a triple hash in bytes.
pub const TRIPLE_HASH_LEN: usize = blake3::OUT_LEN;

/// The all-zero bytes of a triple hash.
pub const TRIPLE_HASH_ZERO: [u8; TRIPLE_HASH_LEN] = [0u8; TRIPLE_HASH_LEN];

/// A cryptographically-secure hash of a triple.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TripleHash(Hash);

impl TripleHash {
    /// The all-zero triple hash.
    pub const ZERO: Self = Self::from_bytes(TRIPLE_HASH_ZERO);

    pub const fn from_bytes(input: [u8; TRIPLE_HASH_LEN]) -> Self {
        Self(Hash::from_bytes(input))
    }

    pub fn as_bytes(&self) -> &[u8; TRIPLE_HASH_LEN] {
        self.0.as_bytes()
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl core::fmt::Display for TripleHash {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl Default for TripleHash {
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<[u8; TRIPLE_HASH_LEN]> for TripleHash {
    fn from(input: [u8; TRIPLE_HASH_LEN]) -> Self {
        Self::from_bytes(input)
    }
}

impl From<&[u8; TRIPLE_HASH_LEN]> for TripleHash {
    fn from(input: &[u8; TRIPLE_HASH_LEN]) -> Self {
        Self::from_bytes(*input)
    }
}

impl From<TripleHash> for [u8; TRIPLE_HASH_LEN] {
    fn from(input: TripleHash) -> Self {
        input.0.into()
    }
}

impl From<Hash> for TripleHash {
    fn from(input: Hash) -> Self {
        Self(input)
    }
}

impl From<&Hash> for TripleHash {
    fn from(input: &Hash) -> Self {
        Self(input.clone())
    }
}

impl From<TripleHash> for Hash {
    fn from(input: TripleHash) -> Self {
        input.0
    }
}

impl From<&TripleHash> for Hash {
    fn from(input: &TripleHash) -> Self {
        input.0.clone()
    }
}

impl From<(TermHash, TermHash, TermHash)> for TripleHash {
    fn from((s, p, o): (TermHash, TermHash, TermHash)) -> Self {
        (&s, &p, &o).into()
    }
}

impl From<(&TermHash, &TermHash, &TermHash)> for TripleHash {
    fn from((s, p, o): (&TermHash, &TermHash, &TermHash)) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(s.as_slice());
        hasher.update(p.as_slice());
        hasher.update(o.as_slice());
        Self(hasher.finalize())
    }
}

impl From<(TermHash, TermHash, TermHash, TermHash)> for TripleHash {
    fn from((s, p, o, _): (TermHash, TermHash, TermHash, TermHash)) -> Self {
        (&s, &p, &o).into()
    }
}

impl From<(&TermHash, &TermHash, &TermHash, &TermHash)> for TripleHash {
    fn from((s, p, o, _): (&TermHash, &TermHash, &TermHash, &TermHash)) -> Self {
        (s, p, o).into()
    }
}

impl From<(CowTerm<'_>, CowTerm<'_>, CowTerm<'_>)> for TripleHash {
    fn from((s, p, o): (CowTerm<'_>, CowTerm<'_>, CowTerm<'_>)) -> Self {
        (TermHash::from(s), TermHash::from(p), TermHash::from(o)).into()
    }
}

impl From<(&CowTerm<'_>, &CowTerm<'_>, &CowTerm<'_>)> for TripleHash {
    fn from((s, p, o): (&CowTerm<'_>, &CowTerm<'_>, &CowTerm<'_>)) -> Self {
        (TermHash::from(s), TermHash::from(p), TermHash::from(o)).into()
    }
}

impl From<(CowTerm<'_>, CowTerm<'_>, CowTerm<'_>, CowTerm<'_>)> for TripleHash {
    fn from((s, p, o, _): (CowTerm<'_>, CowTerm<'_>, CowTerm<'_>, CowTerm<'_>)) -> Self {
        (TermHash::from(s), TermHash::from(p), TermHash::from(o)).into()
    }
}

impl From<(&CowTerm<'_>, &CowTerm<'_>, &CowTerm<'_>, &CowTerm<'_>)> for TripleHash {
    fn from((s, p, o, _): (&CowTerm<'_>, &CowTerm<'_>, &CowTerm<'_>, &CowTerm<'_>)) -> Self {
        (TermHash::from(s), TermHash::from(p), TermHash::from(o)).into()
    }
}

impl From<(HeapTerm, HeapTerm, HeapTerm)> for TripleHash {
    fn from((s, p, o): (HeapTerm, HeapTerm, HeapTerm)) -> Self {
        (TermHash::from(s), TermHash::from(p), TermHash::from(o)).into()
    }
}

impl From<(&HeapTerm, &HeapTerm, &HeapTerm)> for TripleHash {
    fn from((s, p, o): (&HeapTerm, &HeapTerm, &HeapTerm)) -> Self {
        (TermHash::from(s), TermHash::from(p), TermHash::from(o)).into()
    }
}

impl From<(HeapTerm, HeapTerm, HeapTerm, HeapTerm)> for TripleHash {
    fn from((s, p, o, _): (HeapTerm, HeapTerm, HeapTerm, HeapTerm)) -> Self {
        (TermHash::from(s), TermHash::from(p), TermHash::from(o)).into()
    }
}

impl From<(&HeapTerm, &HeapTerm, &HeapTerm, &HeapTerm)> for TripleHash {
    fn from((s, p, o, _): (&HeapTerm, &HeapTerm, &HeapTerm, &HeapTerm)) -> Self {
        (TermHash::from(s), TermHash::from(p), TermHash::from(o)).into()
    }
}
