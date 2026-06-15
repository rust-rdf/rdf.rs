// This is free and unencumbered software released into the public domain.

use hdt::hdt_graph::HdtTerm;

/// A triple statement (S, P, O) originating from an HDT file.
pub type HdtTriple = [HdtTerm; 3];

//pub type HdtTriple = rdf_model::interop::SophiaStatement;
