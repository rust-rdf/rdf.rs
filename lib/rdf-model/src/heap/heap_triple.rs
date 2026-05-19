// This is free and unencumbered software released into the public domain.

use crate::{HeapTerm, Triple};

/// A heap-allocated triple statement.
pub type HeapTriple = Triple<HeapTerm>;
