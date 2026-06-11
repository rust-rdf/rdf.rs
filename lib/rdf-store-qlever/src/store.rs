// This is free and unencumbered software released into the public domain.

use crate::{QleverError, QleverTransaction};
use alloc::boxed::Box;
use derive_more::Debug;
use rdf_store::Store;

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A quad store backed by a QLever (`libqlever`) database.
#[derive(Debug, Default)]
pub struct QleverStore {}

impl QleverStore {}

impl Store for QleverStore {
    type Error = QleverError;
    type Read = QleverTransaction;
    type Write = QleverTransaction;

    async fn read(&mut self) -> Result<Self::Read, Self::Error> {
        QleverTransaction::begin(self, false).await
    }

    async fn write(&mut self) -> Result<Self::Write, Self::Error> {
        QleverTransaction::begin(self, true).await
    }
}
