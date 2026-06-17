// This is free and unencumbered software released into the public domain.

/// The result type for COTTAS writer operations.
pub type CottasWriterResult<T> = core::result::Result<T, CottasWriterError>;

/// An error encountered while writing a COTTAS file.
pub type CottasWriterError = std::io::Error;
