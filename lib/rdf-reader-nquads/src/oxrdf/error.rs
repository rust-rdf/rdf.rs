// This is free and unencumbered software released into the public domain.

pub type NquadsReaderResult<T> = core::result::Result<T, NquadsReaderError>;

/// An error encountered while reading an N-Quads file.
pub type NquadsReaderError = oxttl::TurtleParseError;
