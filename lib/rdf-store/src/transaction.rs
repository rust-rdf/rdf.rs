// This is free and unencumbered software released into the public domain.

use rdf_model::Statement;

pub trait Transaction {
    type Error;

    fn insert_statement(&mut self, statement: &dyn Statement) -> Result<(), Self::Error>;

    fn remove_statement(&mut self, statement: &dyn Statement) -> Result<(), Self::Error>;

    fn commit(&mut self) -> Result<(), Self::Error>;

    fn rollback(&mut self) -> Result<(), Self::Error>;
}

impl core::fmt::Debug for dyn Transaction<Error = core::convert::Infallible> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Transaction").finish()
    }
}
