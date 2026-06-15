// This is free and unencumbered software released into the public domain.

pub type NtriplesReaderResult<T> = core::result::Result<T, NtriplesReaderError>;

/// An error encountered while reading an N-Triples file.
pub type NtriplesReaderError = oxttl::TurtleParseError;
