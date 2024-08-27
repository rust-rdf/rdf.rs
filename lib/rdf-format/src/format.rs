// This is free and unencumbered software released into the public domain.

use crate::prelude::{vec, Vec};

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

impl Format {
    pub fn is_binary(&self) -> bool {
        use Format::*;
        matches!(self, Hdt)
    }

    pub fn is_text(&self) -> bool {
        !self.is_binary()
    }

    pub fn label(&self) -> &str {
        use Format::*;
        match self {
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
        }
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
        f.write_str(self.label())
    }
}
