// This is free and unencumbered software released into the public domain.

pub type HdtReaderResult<T> = core::result::Result<T, HdtReaderError>;

/// An error encountered while reading an HDT file.
pub type HdtReaderError = hdt::hdt::Error;
