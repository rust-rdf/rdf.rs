// This is free and unencumbered software released into the public domain.

/// The result type for RDF/XML writer operations.
pub type RdfxmlWriterResult<T> = core::result::Result<T, RdfxmlWriterError>;

/// An error encountered while writing an RDF/XML file.
pub type RdfxmlWriterError = std::io::Error;
