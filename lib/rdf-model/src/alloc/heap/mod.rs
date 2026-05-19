// This is free and unencumbered software released into the public domain.

use crate::{Quad, QuadPattern, Triple, TriplePattern};

/// A heap-allocated triple statement.
pub type HeapTriple = Triple<HeapTerm>;

/// A heap-allocated triple statement pattern.
pub type HeapTriplePattern = TriplePattern<HeapTerm>;

/// A heap-allocated quad statement.
pub type HeapQuad = Quad<HeapTerm>;

/// A heap-allocated quad statement pattern.
pub type HeapQuadPattern = QuadPattern<HeapTerm>;

mod term;
pub use term::*;
