// This is free and unencumbered software released into the public domain.

/// The result type for N-Triples writer operations.
pub type NtriplesWriterResult<T> = core::result::Result<T, NtriplesWriterError>;

/// An error encountered while writing an N-Triples file.
pub type NtriplesWriterError = std::io::Error;
