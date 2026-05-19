// This is free and unencumbered software released into the public domain.

use crate::{HeapTerm, Quad};

/// A heap-allocated quad statement.
pub type HeapQuad = Quad<HeapTerm>;
