// This is free and unencumbered software released into the public domain.

pub type CottasReaderResult<T> = core::result::Result<T, CottasReaderError>;

/// An error encountered while reading a COTTAS file.
pub type CottasReaderError = ();
