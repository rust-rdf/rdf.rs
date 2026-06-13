// This is free and unencumbered software released into the public domain.

use alloc::{borrow::Cow, vec, vec::Vec};
use dogma::traits::{Labeled, Named};

pub const FORMATS: [(&'static str, Format); 13] = [
    ("csvw", Format::Csvw),
    ("hdt", Format::Hdt),
    ("jelly", Format::Jelly),
    ("jsonld", Format::JsonLd),
    ("n3", Format::Notation3),
    ("nq", Format::NQuads),
    ("nt", Format::NTriples),
    ("rj", Format::RdfJson),
    ("rdf", Format::RdfXml),
    ("trig", Format::TriG),
    ("trix", Format::TriX),
    ("ttl", Format::Turtle),
    ("yamlld", Format::YamlLd),
];

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum Format {
    Csvw,
    Hdt,
    Jelly,
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
    fn name(&self) -> Cow<'_, str> {
        use Format::*;
        Cow::Borrowed(match self {
            Csvw => "csvw",
            Hdt => "hdt",
            Jelly => "jelly",
            JsonLd => "json-ld",
            Notation3 => "n3",
            NQuads => "n-quads",
            NTriples => "n-triples",
            RdfJson => "rdfjson",
            RdfXml => "rdfxml",
            TriG => "trig",
            TriX => "trix",
            Turtle => "turtle",
            YamlLd => "yaml-ld",
        })
    }
}

impl Labeled for Format {
    fn label(&self) -> Cow<'_, str> {
        use Format::*;
        Cow::Borrowed(match self {
            Csvw => "CSVW",
            Hdt => "HDT",
            Jelly => "Jelly",
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

impl Format {
    pub fn from_extension(extension: &str) -> Option<Self> {
        for (format_extension, format) in FORMATS {
            if extension.eq_ignore_ascii_case(format_extension) {
                return Some(format);
            }
        }
        None
    }

    pub fn extension(&self) -> &str {
        use Format::*;
        match self {
            Csvw => "csv",
            Hdt => "hdt",
            Jelly => "jelly",
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

    pub fn is_binary(&self) -> bool {
        use Format::*;
        matches!(self, Hdt | Jelly)
    }

    pub fn is_text(&self) -> bool {
        !self.is_binary()
    }
}

impl core::fmt::Display for Format {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(&self.label())
    }
}

#[cfg(feature = "oxrdf")]
impl TryFrom<&oxrdfio::RdfFormat> for Format {
    type Error = ();

    fn try_from(format: &oxrdfio::RdfFormat) -> Result<Self, Self::Error> {
        use oxrdfio::RdfFormat;
        Ok(match format {
            RdfFormat::N3 => Format::Notation3,
            RdfFormat::NQuads => Format::NQuads,
            RdfFormat::NTriples => Format::NTriples,
            RdfFormat::RdfXml => Format::RdfXml,
            RdfFormat::TriG => Format::TriG,
            RdfFormat::Turtle => Format::Turtle,
            RdfFormat::JsonLd { .. } => Format::JsonLd,
            _ => return Err(()),
        })
    }
}

#[cfg(feature = "oxrdf")]
impl TryInto<oxrdfio::RdfFormat> for Format {
    type Error = ();

    fn try_into(self) -> Result<oxrdfio::RdfFormat, Self::Error> {
        TryInto::<oxrdfio::RdfFormat>::try_into(&self)
    }
}

#[cfg(feature = "oxrdf")]
impl TryInto<oxrdfio::RdfFormat> for &Format {
    type Error = ();

    fn try_into(self) -> Result<oxrdfio::RdfFormat, Self::Error> {
        use oxrdfio::RdfFormat;
        Ok(match self {
            Format::Notation3 => RdfFormat::N3,
            Format::NQuads => RdfFormat::NQuads,
            Format::NTriples => RdfFormat::NTriples,
            Format::RdfXml => RdfFormat::RdfXml,
            Format::TriG => RdfFormat::TriG,
            Format::Turtle => RdfFormat::Turtle,
            _ => return Err(()),
        })
    }
}
