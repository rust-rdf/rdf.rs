// This is free and unencumbered software released into the public domain.

use borsh::{BorshDeserialize, BorshSerialize};

pub(crate) const MAGIC_NUMBER: [u8; 4] = *b"RDFB";
pub(crate) const VERSION_NUMBER: u8 = b'1';
pub(crate) const FLAGS: u8 = 0b00000111;

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    BorshSerialize,
    BorshDeserialize,
)]
pub struct BorshHeader {
    pub magic: [u8; 4],
    pub version: u8,
    pub flags: u8,
    pub quad_count: u32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BorshHeaderError {
    InvalidMagic,
    InvalidVersion,
    InvalidFlags,
}

impl BorshHeaderError {
    pub fn check(header: &BorshHeader) -> Result<(), Self> {
        if header.magic != MAGIC_NUMBER {
            return Err(Self::InvalidMagic);
        }
        if header.version != VERSION_NUMBER {
            return Err(Self::InvalidVersion);
        }
        if header.flags != FLAGS {
            return Err(Self::InvalidFlags);
        }
        Ok(())
    }
}

impl core::error::Error for BorshHeaderError {}

impl core::fmt::Display for BorshHeaderError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::InvalidMagic => write!(f, "invalid RDF/Borsh header"),
            Self::InvalidVersion => write!(f, "invalid RDF/Borsh version"),
            Self::InvalidFlags => write!(f, "invalid RDF/Borsh flags"),
        }
    }
}
