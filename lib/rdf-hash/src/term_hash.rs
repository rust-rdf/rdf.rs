// This is free and unencumbered software released into the public domain.

pub const TERM_HASH_LEN: usize = blake3::OUT_LEN;

pub type TermHash = blake3::Hash;
