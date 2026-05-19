// This is free and unencumbered software released into the public domain.

use crate::CowTerm;

pub const RDF_TYPE: CowTerm<'static> =
    CowTerm::iri_static("http://www.w3.org/1999/02/22-rdf-syntax-ns#type");

pub const RDFS_LABEL: CowTerm<'static> =
    CowTerm::iri_static("http://www.w3.org/2000/01/rdf-schema#label");
