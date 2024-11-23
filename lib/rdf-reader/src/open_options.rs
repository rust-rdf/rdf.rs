// This is free and unencumbered software released into the public domain.

use rdf_format::Format;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OpenOptions {
    pub format: Option<Format>,
}
