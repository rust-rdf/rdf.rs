// This is free and unencumbered software released into the public domain.

#[cfg(feature = "alloc")]
use crate::{CowTerm, HeapTerm};
use crate::{
    DEFAULT_GRAPH, DefaultGraph, QuadPattern, Statement, StatementPattern, Term, Triple,
    TriplePattern,
};
use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

pub type QuadSlot = crate::StatementSlot;

/// A quad statement.
///
/// A `None` context denotes the default graph. An explicit default-graph marker
/// is accepted as an alias: accessors, extraction, equality, ordering, hashing,
/// and serialization use the canonical `None` context. Pattern conversion instead
/// uses `Some(DEFAULT_GRAPH.into())` to distinguish this graph from a wildcard.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Quad<T: Term> {
    pub(crate) s: T,
    pub(crate) p: T,
    pub(crate) o: T,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_context"))]
    pub(crate) g: Option<T>,
}

impl<T: Term + PartialEq> PartialEq for Quad<T> {
    fn eq(&self, other: &Self) -> bool {
        (&self.s, &self.p, &self.o, self.context())
            == (&other.s, &other.p, &other.o, other.context())
    }
}

impl<T: Term + Eq> Eq for Quad<T> {}

impl<T: Term + PartialOrd> PartialOrd for Quad<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (&self.s, &self.p, &self.o, self.context()).partial_cmp(&(
            &other.s,
            &other.p,
            &other.o,
            other.context(),
        ))
    }
}

impl<T: Term + Ord> Ord for Quad<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.s, &self.p, &self.o, self.context()).cmp(&(
            &other.s,
            &other.p,
            &other.o,
            other.context(),
        ))
    }
}

impl<T: Term + Hash> Hash for Quad<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.s.hash(state);
        self.p.hash(state);
        self.o.hash(state);
        self.context().hash(state);
    }
}

#[cfg(feature = "serde")]
fn serialize_context<T: Term + serde::Serialize, S: serde::Serializer>(
    context: &Option<T>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serde::Serialize::serialize(
        &context.as_ref().filter(|g| !g.is_default_graph()),
        serializer,
    )
}

impl<T: Term> Quad<T> {
    /// Constructs a quad. `None` or the default-graph singleton denotes the
    /// default graph; a different term denotes its named graph.
    pub const fn new(s: T, p: T, o: T, g: Option<T>) -> Self {
        Self { s, p, o, g }
    }

    pub fn with_subject(self, s: impl Into<T>) -> Self {
        Self {
            s: s.into(),
            ..self
        }
    }

    pub fn with_predicate(self, p: impl Into<T>) -> Self {
        Self {
            p: p.into(),
            ..self
        }
    }

    pub fn with_object(self, o: impl Into<T>) -> Self {
        Self {
            o: o.into(),
            ..self
        }
    }

    /// Replaces the graph, normalizing a default-graph marker to `None`.
    pub fn with_context(self, g: impl Into<Option<T>>) -> Self {
        Self {
            g: g.into().filter(|term| !term.is_default_graph()),
            ..self
        }
    }

    /// Extracts the statement, returning `None` for the default graph, including
    /// an explicitly supplied singleton alias.
    pub fn into_inner(self) -> (T, T, T, Option<T>) {
        (
            self.s,
            self.p,
            self.o,
            self.g.filter(|term| !term.is_default_graph()),
        )
    }

    /// Whether the statement has a constant subject.
    pub fn has_subject(&self) -> bool {
        true
    }

    /// Whether the statement has a constant predicate.
    pub fn has_predicate(&self) -> bool {
        true
    }

    /// Whether the statement has a constant object.
    pub fn has_object(&self) -> bool {
        true
    }

    /// Whether this quad has a named graph. Both representations of the default
    /// graph return `false`; as a pattern, the quad still constrains its graph.
    pub fn has_context(&self) -> bool {
        self.context().is_some()
    }

    pub fn subject(&self) -> &T {
        &self.s
    }

    pub fn predicate(&self) -> &T {
        &self.p
    }

    pub fn object(&self) -> &T {
        &self.o
    }

    /// Returns the graph name, or `None` for the default graph. The singleton
    /// alias is normalized without allocation or mutation.
    pub fn context(&self) -> Option<&T> {
        self.g.as_ref().filter(|term| !term.is_default_graph())
    }
}

impl<T: Term + Clone> Statement for Quad<T> {
    type Term = T;

    fn subject(&self) -> &Self::Term {
        &self.s
    }

    fn predicate(&self) -> &Self::Term {
        &self.p
    }

    fn object(&self) -> &Self::Term {
        &self.o
    }

    fn context(&self) -> Option<&Self::Term> {
        self.context()
    }
}

impl<T: Term + Clone> StatementPattern for Quad<T> {
    type Term = T;

    fn subject(&self) -> Option<&Self::Term> {
        Some(&self.s)
    }

    fn predicate(&self) -> Option<&Self::Term> {
        Some(&self.p)
    }

    fn object(&self) -> Option<&Self::Term> {
        Some(&self.o)
    }

    fn context(&self) -> Option<&Self::Term> {
        self.g.as_ref()
    }

    fn is_default_graph(&self) -> bool {
        self.context().is_none()
    }
}

impl<T: Term + Clone> Quad<T> {
    pub fn to_triple(&self) -> Triple<T> {
        Triple::new(self.s.clone(), self.p.clone(), self.o.clone())
    }

    pub fn to_triple_pattern(&self) -> TriplePattern<T> {
        TriplePattern::new(
            Some(self.s.clone()),
            Some(self.p.clone()),
            Some(self.o.clone()),
        )
    }

    /// Produces an exact pattern, preserving the default graph via its singleton.
    /// The term representation must support `From<DefaultGraph>`.
    pub fn to_quad_pattern(&self) -> QuadPattern<T>
    where
        T: From<DefaultGraph>,
    {
        QuadPattern::new(
            Some(self.s.clone()),
            Some(self.p.clone()),
            Some(self.o.clone()),
            Some(
                self.context()
                    .cloned()
                    .unwrap_or_else(|| DEFAULT_GRAPH.into()),
            ),
        )
    }
}

#[cfg(feature = "alloc")]
impl From<Triple<CowTerm<'_>>> for Quad<HeapTerm> {
    fn from(input: Triple<CowTerm<'_>>) -> Self {
        Self::new(input.s.into(), input.p.into(), input.o.into(), None)
    }
}

#[cfg(feature = "alloc")]
impl<'a> From<&'a Triple<CowTerm<'a>>> for Quad<HeapTerm> {
    fn from(input: &'a Triple<CowTerm<'a>>) -> Self {
        Self::new(
            (&input.s).into(),
            (&input.p).into(),
            (&input.o).into(),
            None,
        )
    }
}

#[cfg(feature = "alloc")]
impl From<Quad<CowTerm<'_>>> for Quad<HeapTerm> {
    fn from(input: Quad<CowTerm<'_>>) -> Self {
        Self::new(
            input.s.into(),
            input.p.into(),
            input.o.into(),
            input.g.map(|g| g.into()),
        )
    }
}

#[cfg(feature = "alloc")]
impl<'a> From<&'a Quad<CowTerm<'a>>> for Quad<HeapTerm> {
    fn from(input: &'a Quad<CowTerm<'a>>) -> Self {
        Self::new(
            (&input.s).into(),
            (&input.p).into(),
            (&input.o).into(),
            input.g.as_ref().map(|g| g.into()),
        )
    }
}

#[cfg(feature = "alloc")]
impl<'a> From<Triple<HeapTerm>> for Quad<CowTerm<'a>> {
    fn from(input: Triple<HeapTerm>) -> Self {
        Self::new(input.s.into(), input.p.into(), input.o.into(), None)
    }
}

#[cfg(feature = "alloc")]
impl<'a> From<&'a Triple<HeapTerm>> for Quad<CowTerm<'a>> {
    fn from(input: &'a Triple<HeapTerm>) -> Self {
        Self::new(
            (&input.s).into(),
            (&input.p).into(),
            (&input.o).into(),
            None,
        )
    }
}

#[cfg(feature = "alloc")]
impl<'a> From<Quad<HeapTerm>> for Quad<CowTerm<'a>> {
    fn from(input: Quad<HeapTerm>) -> Self {
        Self::new(
            input.s.into(),
            input.p.into(),
            input.o.into(),
            input.g.map(Into::into),
        )
    }
}

#[cfg(feature = "alloc")]
impl<'a> From<&'a Quad<HeapTerm>> for Quad<CowTerm<'a>> {
    fn from(input: &'a Quad<HeapTerm>) -> Self {
        Self::new(
            (&input.s).into(),
            (&input.p).into(),
            (&input.o).into(),
            input.g.as_ref().map(|g| g.into()),
        )
    }
}

impl<T: Term> From<(T, T, T)> for Quad<T> {
    fn from((s, p, o): (T, T, T)) -> Self {
        Self::new(s, p, o, None)
    }
}

impl<T: Term + Clone> From<(&T, &T, &T)> for Quad<T> {
    fn from((s, p, o): (&T, &T, &T)) -> Self {
        Self::new(s.clone(), p.clone(), o.clone(), None)
    }
}

impl<T: Term> From<(T, T, T, T)> for Quad<T> {
    fn from((s, p, o, g): (T, T, T, T)) -> Self {
        Self::new(s, p, o, Some(g))
    }
}

impl<T: Term> From<(T, T, T, Option<T>)> for Quad<T> {
    fn from((s, p, o, g): (T, T, T, Option<T>)) -> Self {
        Self::new(s, p, o, g)
    }
}

impl<T: Term + Clone> From<(&T, &T, &T, &T)> for Quad<T> {
    fn from((s, p, o, g): (&T, &T, &T, &T)) -> Self {
        Self::new(s.clone(), p.clone(), o.clone(), Some(g.clone()))
    }
}

impl<T: Term + Clone> From<(&T, &T, &T, &Option<T>)> for Quad<T> {
    fn from((s, p, o, g): (&T, &T, &T, &Option<T>)) -> Self {
        Self::new(s.clone(), p.clone(), o.clone(), g.clone())
    }
}

impl<T: Term + Clone> From<(&T, &T, &T, Option<&T>)> for Quad<T> {
    fn from((s, p, o, g): (&T, &T, &T, Option<&T>)) -> Self {
        Self::new(s.clone(), p.clone(), o.clone(), g.cloned())
    }
}

impl<T: Term> TryFrom<TriplePattern<T>> for Quad<T> {
    type Error = ();

    fn try_from(input: TriplePattern<T>) -> Result<Self, Self::Error> {
        if !input.is_constant() {
            return Err(());
        }
        Ok(Self::new(
            input.s.unwrap(),
            input.p.unwrap(),
            input.o.unwrap(),
            None,
        ))
    }
}

/// Extracts a fully bound statement. Wildcard graph patterns are rejected;
/// a singleton-bound default graph becomes the concrete `None` context.
impl<T: Term> TryFrom<QuadPattern<T>> for Quad<T> {
    type Error = ();

    fn try_from(input: QuadPattern<T>) -> Result<Self, Self::Error> {
        if !input.is_constant() {
            return Err(());
        }
        Ok(Self::new(
            input.s.unwrap(),
            input.p.unwrap(),
            input.o.unwrap(),
            input.g.filter(|term| !term.is_default_graph()),
        ))
    }
}

impl<T: Term> FromIterator<T> for Quad<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        Self::new(
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            Some(iter.next().unwrap()),
        )
    }
}
