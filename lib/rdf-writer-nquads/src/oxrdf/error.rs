// This is free and unencumbered software released into the public domain.

/// The result type for N-Quads writer operations.
pub type NquadsWriterResult<T> = core::result::Result<T, NquadsWriterError>;

/// An error encountered while writing an N-Quads file.
pub type NquadsWriterError = std::io::Error;
