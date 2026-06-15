// This is free and unencumbered software released into the public domain.

pub type TurtleReaderResult<T> = core::result::Result<T, TurtleReaderError>;

/// An error encountered while reading a Turtle file.
pub type TurtleReaderError = oxttl::TurtleParseError;
