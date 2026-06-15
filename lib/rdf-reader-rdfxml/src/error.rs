// This is free and unencumbered software released into the public domain.

pub type RdfxmlReaderResult<T> = core::result::Result<T, RdfxmlReaderError>;

/// An error encountered while reading a RDF/XML file.
pub type RdfxmlReaderError = oxrdfxml::RdfXmlParseError;
