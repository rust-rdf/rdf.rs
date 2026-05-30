// This is free and unencumbered software released into the public domain.

use crate::{Quad, QuadPattern, Triple, TriplePattern};

pub use alloc::borrow::{Cow, ToOwned};

/// A clone-on-write triple statement.
pub type CowTriple<'a> = Triple<CowTerm<'a>>;

/// A clone-on-write triple statement pattern.
pub type CowTriplePattern<'a> = TriplePattern<CowTerm<'a>>;

/// A heap-allocated set of triple statements.
#[cfg(feature = "alloc")]
pub type CowTripleSet<'a> = alloc::collections::BTreeSet<CowTriple<'a>>;

/// A clone-on-write quad statement.
pub type CowQuad<'a> = Quad<CowTerm<'a>>;

/// A clone-on-write quad statement pattern.
pub type CowQuadPattern<'a> = QuadPattern<CowTerm<'a>>;

/// A heap-allocated set of quad statements.
#[cfg(feature = "alloc")]
pub type CowQuadSet<'a> = alloc::collections::BTreeSet<CowQuad<'a>>;

mod r#const;
pub use r#const::*;

mod sample;
pub use sample::*;

mod term;
pub use term::*;
