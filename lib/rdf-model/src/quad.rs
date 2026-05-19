// This is free and unencumbered software released into the public domain.

use crate::{Statement, Term};

/// A quad statement.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Quad<T: Term> {
    s: T,
    p: T,
    o: T,
    g: Option<T>,
}

impl<T: Term> Statement for Quad<T> {
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
        self.g.as_ref()
    }
}

impl<T: Term> From<(T, T, T)> for Quad<T> {
    fn from((s, p, o): (T, T, T)) -> Self {
        Self { s, p, o, g: None }
    }
}

impl<T: Term + Clone> From<(&T, &T, &T)> for Quad<T> {
    fn from((s, p, o): (&T, &T, &T)) -> Self {
        Self {
            s: s.clone(),
            p: p.clone(),
            o: o.clone(),
            g: None,
        }
    }
}

impl<T: Term> From<(T, T, T, T)> for Quad<T> {
    fn from((s, p, o, g): (T, T, T, T)) -> Self {
        Self {
            s,
            p,
            o,
            g: Some(g),
        }
    }
}

impl<T: Term + Clone> From<(&T, &T, &T, &T)> for Quad<T> {
    fn from((s, p, o, g): (&T, &T, &T, &T)) -> Self {
        Self {
            s: s.clone(),
            p: p.clone(),
            o: o.clone(),
            g: Some(g.clone()),
        }
    }
}
