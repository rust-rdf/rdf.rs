// This is free and unencumbered software released into the public domain.

pub type JsonldReaderResult<T> = core::result::Result<T, JsonldReaderError>;

/// An error encountered while reading a JSON-LD document.
pub type JsonldReaderError = oxjsonld::JsonLdParseError;
