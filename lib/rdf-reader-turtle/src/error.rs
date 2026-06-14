// This is free and unencumbered software released into the public domain.

pub type TurtleReaderError = oxttl::TurtleParseError;

pub type TurtleReaderResult<T> = core::result::Result<T, TurtleReaderError>;
