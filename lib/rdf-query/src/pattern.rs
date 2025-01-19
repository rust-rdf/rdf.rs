use rdf_model::Statement;

pub struct Pattern {}

impl Statement for Pattern {
    fn subject(&self) -> &dyn rdf_model::Term {
        todo!()
    }

    fn predicate(&self) -> &dyn rdf_model::Term {
        todo!()
    }

    fn object(&self) -> &dyn rdf_model::Term {
        todo!()
    }
}

impl core::fmt::Debug for Pattern {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pattern").finish()
    }
}
