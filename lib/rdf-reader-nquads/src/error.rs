// This is free and unencumbered software released into the public domain.

pub type NquadsReaderError = oxttl::TurtleParseError;

pub type NquadsReaderResult<T> = core::result::Result<T, NquadsReaderError>;
