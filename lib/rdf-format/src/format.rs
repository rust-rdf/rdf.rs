// This is free and unencumbered software released into the public domain.

extern crate alloc;

use crate::prelude::{vec, Vec};
use alloc::borrow::Cow;
use dogma::traits::{Labeled, Named};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Format {
    Hdt,
    JsonLd,
    Notation3,
    NQuads,
    NTriples,
    RdfJson,
    RdfXml,
    TriG,
    TriX,
    Turtle,
    YamlLd,
}

impl Named for Format {
    fn name(&self) -> Cow<str> {
        use Format::*;
        Cow::Borrowed(match self {
            Hdt => "HDT",
            JsonLd => "JSON-LD",
            Notation3 => "Notation3",
            NQuads => "N-Quads",
            NTriples => "N-Triples",
            RdfJson => "RDF/JSON",
            RdfXml => "RDF/XML",
            TriG => "TriG",
            TriX => "TriX",
            Turtle => "Turtle",
            YamlLd => "YAML-LD",
        })
    }
}

impl Labeled for Format {
    fn label(&self) -> Cow<str> {
        self.name()
    }
}

impl Format {
    pub fn is_binary(&self) -> bool {
        use Format::*;
        matches!(self, Hdt)
    }

    pub fn is_text(&self) -> bool {
        !self.is_binary()
    }

    pub fn extension(&self) -> &str {
        use Format::*;
        match self {
            Hdt => "hdt",
            JsonLd => "jsonld",
            Notation3 => "n3",
            NQuads => "nq",
            NTriples => "nt",
            RdfJson => "rj",
            RdfXml => "rdf",
            TriG => "trig",
            TriX => "trix",
            Turtle => "ttl",
            YamlLd => "yamlld",
        }
    }

    pub fn extensions(&self) -> Vec<&str> {
        use Format::*;
        match self {
            YamlLd => vec!["yamlld", "yaml", "yml"],
            _ => vec![self.extension()],
        }
    }
}

impl core::fmt::Display for Format {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(&self.label())
    }
}
