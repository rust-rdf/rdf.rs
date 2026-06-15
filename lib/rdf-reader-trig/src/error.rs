// This is free and unencumbered software released into the public domain.

pub type TrigReaderResult<T> = core::result::Result<T, TrigReaderError>;

/// An error encountered while reading a TriG file.
pub type TrigReaderError = oxttl::TurtleParseError;
