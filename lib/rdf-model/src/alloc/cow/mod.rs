// This is free and unencumbered software released into the public domain.

extern crate alloc;

use crate::{Quad, QuadPattern, Triple, TriplePattern};

pub use alloc::borrow::{Cow, ToOwned};

/// A clone-on-write triple statement.
pub type CowTriple<'a> = Triple<CowTerm<'a>>;

/// A clone-on-write triple statement pattern.
pub type CowTriplePattern<'a> = TriplePattern<CowTerm<'a>>;

/// A clone-on-write quad statement.
pub type CowQuad<'a> = Quad<CowTerm<'a>>;

/// A clone-on-write quad statement pattern.
pub type CowQuadPattern<'a> = QuadPattern<CowTerm<'a>>;

mod r#const;
pub use r#const::*;

mod sample;
pub use sample::*;

mod term;
pub use term::*;
