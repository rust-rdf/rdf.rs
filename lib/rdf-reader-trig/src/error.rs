// This is free and unencumbered software released into the public domain.

pub type TrigReaderError = oxttl::TurtleParseError;

pub type TrigReaderResult<T> = core::result::Result<T, TrigReaderError>;
