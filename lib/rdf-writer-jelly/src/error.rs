// This is free and unencumbered software released into the public domain.

/// The result type for Jelly writer operations.
pub type JellyWriterResult<T> = core::result::Result<T, JellyWriterError>;

/// An error encountered while writing a Jelly file.
pub type JellyWriterError = std::io::Error;
