// This is free and unencumbered software released into the public domain.

use crate::{HeapTerm, QuadPattern};

/// A heap-allocated quad statement pattern.
pub type HeapQuadPattern = QuadPattern<HeapTerm>;
