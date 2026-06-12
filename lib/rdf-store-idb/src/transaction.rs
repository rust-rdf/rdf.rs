// This is free and unencumbered software released into the public domain.

use crate::{IdbError, IdbStore};
use core::ops::{Deref, DerefMut};
use derive_more::Debug;
use futures::{Stream, stream};
use rdf_model::{HeapQuad, HeapQuadPattern, HeapTerm};
use rdf_store::{ReadTransaction, WriteTransaction};
use send_wrapper::SendWrapper;

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A transaction for reading and writing statements in IndexedDB.
#[derive(Debug)]
pub struct IdbTransaction {
    writable: bool,
    tx: SendWrapper<idb::Transaction>,
    objects: idb::ObjectStore,
}

// Static assertion that `IdbTransaction` is `Send`.
const _: () = {
    const fn assert_send<T: Send>() {}
    let _ = assert_send::<IdbTransaction>;
};

impl IdbTransaction {
    pub async fn begin(store: &IdbStore, writable: bool) -> Result<Self, IdbError> {
        let tx = SendWrapper::new(store.db.transaction(
            &[&store.name],
            if writable {
                idb::TransactionMode::ReadWrite
            } else {
                idb::TransactionMode::ReadOnly
            },
        )?);
        let objects = tx.object_store(&store.name)?;
        Ok(Self {
            writable,
            tx,
            objects,
        })
    }

    pub fn is_writable(&self) -> bool {
        self.writable
    }
}

impl WriteTransaction for IdbTransaction {
    type Error = IdbError;
    type Term = HeapTerm;
    type Statement = HeapQuad;
    type StatementPattern = HeapQuadPattern;

    async fn rollback(self) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(IdbError::ReadOnly);
        };

        let tx = self.tx.take();
        tx.abort()?;

        Ok(())
    }

    async fn commit(self) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(IdbError::ReadOnly);
        };

        let tx = self.tx.take();
        tx.commit()?;

        Ok(())
    }

    async fn clear(&mut self) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(IdbError::ReadOnly);
        };

        self.objects.clear()?;

        Ok(())
    }

    async fn insert(
        &mut self,
        _statement: impl Into<Self::Statement> + Send,
    ) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(IdbError::ReadOnly);
        };

        use wasm_bindgen::{JsCast, JsValue};
        let k = JsValue::from_str("mykey");
        let v = JsValue::from_str("myval");
        self.objects.put(&v, Some(&k))?;

        Ok(()) // TODO
    }

    async fn remove(
        &mut self,
        _statement: impl Into<Self::Statement> + Send,
    ) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(IdbError::ReadOnly);
        };

        Ok(()) // TODO
    }

    async fn delete(
        &mut self,
        _pattern: impl Into<Self::StatementPattern> + Send,
    ) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(IdbError::ReadOnly);
        }

        Ok(()) // TODO
    }
}

impl ReadTransaction for IdbTransaction {
    type Error = IdbError;
    type Term = HeapTerm;
    type Statement = HeapQuad;
    type StatementPattern = HeapQuadPattern;

    fn r#match(
        &self,
        _pattern: impl Into<Self::StatementPattern>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> {
        stream::empty() // TODO
    }
}
