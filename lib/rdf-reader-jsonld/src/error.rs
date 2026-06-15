// This is free and unencumbered software released into the public domain.

pub type JsonldReaderError = oxjsonld::JsonLdParseError;

pub type JsonldReaderResult<T> = core::result::Result<T, JsonldReaderError>;
