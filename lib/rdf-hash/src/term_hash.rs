// This is free and unencumbered software released into the public domain.

use blake3::{Hash, Hasher};
use rdf_model::{CowTerm, HeapTerm};

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

impl core::fmt::Display for TermHash {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.0.fmt(f)
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

impl From<CowTerm<'_>> for TermHash {
    fn from(input: CowTerm<'_>) -> Self {
        (&input).into()
    }
}

impl From<&CowTerm<'_>> for TermHash {
    fn from(input: &CowTerm<'_>) -> Self {
        // See: https://www.w3.org/TR/rdf12-n-triples/
        use heapless::String;
        let mut hasher = Hasher::new();
        match input {
            CowTerm::Iri(str) => {
                hasher.update(b"<");
                hasher.update(str.as_bytes()); // TODO: escaping
                hasher.update(b">");
            },
            CowTerm::BNode(id) => {
                hasher.update(b"_:");
                hasher.update(id.as_bytes());
            },
            CowTerm::String(val) => {
                hasher.update(b"\"");
                hasher.update(val.as_bytes()); // TODO: escaping
                hasher.update(b"\"");
            },
            CowTerm::TaggedString(val, lang, dir) => {
                let mut lang_lower: String<64> = String::new();
                for c in lang.chars() {
                    lang_lower.push(c.to_ascii_lowercase()).unwrap();
                }
                hasher.update(b"\"");
                hasher.update(val.as_bytes()); // TODO: escaping
                hasher.update(b"\"@");
                hasher.update(lang_lower.as_bytes());
                if let Some(dir) = dir {
                    hasher.update(b"--");
                    hasher.update(dir.as_str().as_bytes());
                }
            },
            CowTerm::TypedLiteral(lit, r#type) => {
                hasher.update(b"\"");
                hasher.update(lit.as_bytes()); // TODO: escaping
                hasher.update(b"\"^^<");
                hasher.update(r#type.iri_string().as_bytes());
                hasher.update(b">");
            },
            CowTerm::TypedValue(val) => {
                use alloc::string::ToString; // FIXME
                hasher.update(b"\"");
                hasher.update(val.to_string().as_bytes()); // TODO: escaping
                hasher.update(b"\"^^<");
                hasher.update(val.r#type().iri_string().as_bytes());
                hasher.update(b">");
            },
        };
        Self(hasher.finalize())
    }
}

impl From<HeapTerm> for TermHash {
    fn from(input: HeapTerm) -> Self {
        CowTerm::from(input).into()
    }
}

impl From<&HeapTerm> for TermHash {
    fn from(input: &HeapTerm) -> Self {
        CowTerm::from(input).into()
    }
}
