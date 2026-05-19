// This is free and unencumbered software released into the public domain.

use crate::{Cow, CowQuad, CowTerm, CowTriple, RDFS_LABEL};

/// A sample resource term.
pub const SAMPLE_RESOURCE: CowTerm<'static> = CowTerm::iri_static("https://example.org/resource");

/// A sample plain literal term.
pub const SAMPLE_LITERAL: CowTerm<'static> = CowTerm::literal_static("Example Resource");

/// A sample triple.
pub const SAMPLE_TRIPLE: CowTriple = CowTriple::new(SAMPLE_RESOURCE, RDFS_LABEL, SAMPLE_LITERAL);

/// A sample quad.
pub const SAMPLE_QUAD: CowQuad = CowQuad::new(SAMPLE_RESOURCE, RDFS_LABEL, SAMPLE_LITERAL, None);
