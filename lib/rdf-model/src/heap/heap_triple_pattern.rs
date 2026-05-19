// This is free and unencumbered software released into the public domain.

use crate::{HeapTerm, TriplePattern};

/// A heap-allocated triple statement pattern.
pub type HeapTriplePattern = TriplePattern<HeapTerm>;
