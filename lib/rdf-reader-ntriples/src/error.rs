// This is free and unencumbered software released into the public domain.

pub type NtriplesReaderError = oxttl::TurtleParseError;

pub type NtriplesReaderResult<T> = core::result::Result<T, NtriplesReaderError>;
