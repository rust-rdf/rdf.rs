// This is free and unencumbered software released into the public domain.

/// The result type for TriG writer operations.
pub type TrigWriterResult<T> = core::result::Result<T, TrigWriterError>;

/// An error encountered while writing an TriG file.
pub type TrigWriterError = std::io::Error;
