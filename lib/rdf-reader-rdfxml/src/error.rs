// This is free and unencumbered software released into the public domain.

pub type RdfxmlReaderError = oxrdfxml::RdfXmlParseError;

pub type RdfxmlReaderResult<T> = core::result::Result<T, RdfxmlReaderError>;
