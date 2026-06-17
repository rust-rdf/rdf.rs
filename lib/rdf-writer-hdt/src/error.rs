// This is free and unencumbered software released into the public domain.

/// The result type for HDT writer operations.
pub type HdtWriterResult<T> = core::result::Result<T, HdtWriterError>;

/// An error encountered while writing an HDT file.
pub type HdtWriterError = std::io::Error;
