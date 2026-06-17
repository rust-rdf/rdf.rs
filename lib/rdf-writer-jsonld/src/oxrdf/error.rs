// This is free and unencumbered software released into the public domain.

/// The result type for JSON-LD writer operations.
pub type JsonldWriterResult<T> = core::result::Result<T, JsonldWriterError>;

/// An error encountered while writing a JSON-LD file.
pub type JsonldWriterError = std::io::Error;
