use rdf_model::Statement;

use crate::BorshTerm;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BorshStatement {
    s: BorshTerm,
    p: BorshTerm,
    o: BorshTerm,
    g: BorshTerm,
}

impl Statement for BorshStatement {
    fn subject(&self) -> &dyn rdf_model::Term {
        &self.s
    }

    fn predicate(&self) -> &dyn rdf_model::Term {
        &self.p
    }

    fn object(&self) -> &dyn rdf_model::Term {
        &self.o
    }

    fn context(&self) -> Option<&dyn rdf_model::Term> {
        Some(&self.g)
    }
}

impl From<(BorshTerm, BorshTerm, BorshTerm, BorshTerm)> for BorshStatement {
    fn from((s, p, o, g): (BorshTerm, BorshTerm, BorshTerm, BorshTerm)) -> Self {
        BorshStatement { s, p, o, g }
    }
}
