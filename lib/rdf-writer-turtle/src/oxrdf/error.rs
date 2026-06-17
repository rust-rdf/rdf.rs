// This is free and unencumbered software released into the public domain.

/// The result type for Turtle writer operations.
pub type TurtleWriterResult<T> = core::result::Result<T, TurtleWriterError>;

/// An error encountered while writing a Turtle file.
pub type TurtleWriterError = std::io::Error;
