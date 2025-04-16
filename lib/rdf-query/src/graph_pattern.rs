use crate::{pattern::Pattern, query::Query};

pub enum GraphPattern {
    BasicGraphPattern(Query),
    TriplePattern(Pattern),
}

impl From<Query> for GraphPattern {
    fn from(query: Query) -> Self {
        Self::BasicGraphPattern(query)
    }
}

impl From<Pattern> for GraphPattern {
    fn from(pattern: Pattern) -> Self {
        Self::TriplePattern(pattern)
    }
}
