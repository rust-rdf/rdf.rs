// This is free and unencumbered software released into the public domain.

#[deprecated(since = "0.3.5", note = "Use `rdf_hash::TERM_HASH_LEN`")]
pub const TERM_HASH_LEN: usize = blake3::OUT_LEN;

#[deprecated(since = "0.3.5", note = "Use `rdf_hash::TermHash`")]
pub type TermHash = blake3::Hash;
