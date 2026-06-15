// This is free and unencumbered software released into the public domain.

pub type HdtReaderError = hdt::hdt::Error;

pub type HdtReaderResult<T> = core::result::Result<T, HdtReaderError>;
