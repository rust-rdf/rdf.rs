// This is free and unencumbered software released into the public domain.

use super::{ReadTransaction, WriteTransaction};

/// A store of statements that supports R/O and R/W transactions.
pub trait Store {
    type Error: Send;
    type Read: ReadTransaction<Error = Self::Error> + Send;
    type Write: WriteTransaction<Error = Self::Error> + Send;

    /// Begins a read-only (R/O) transaction.
    fn read(&mut self) -> impl Future<Output = Result<Self::Read, Self::Error>> + Send;

    /// Begins a read-write (R/W) transaction.
    fn write(&mut self) -> impl Future<Output = Result<Self::Write, Self::Error>> + Send;
}
