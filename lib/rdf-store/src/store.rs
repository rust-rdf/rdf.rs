// This is free and unencumbered software released into the public domain.

use super::{ReadTransaction, WriteTransaction};

pub trait Store {
    type Error;
    type Read: ReadTransaction<Error = Self::Error> + Send;
    type Write: WriteTransaction<Error = Self::Error> + Send;

    fn read(&mut self) -> impl Future<Output = Result<Self::Read, Self::Error>>;

    fn write(&mut self) -> impl Future<Output = Result<Self::Write, Self::Error>>;
}
