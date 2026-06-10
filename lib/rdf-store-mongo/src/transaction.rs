// This is free and unencumbered software released into the public domain.

use crate::{MongoError, MongoStore, MongoTriple, MongoTripleId};
use alloc::boxed::Box;
use async_trait::async_trait;
use derive_more::Debug;
use futures::{Stream, stream};
use mongodb::{
    ClientSession, Collection,
    bson::Document,
    error::{ErrorKind, WriteError, WriteFailure},
    options::SessionOptions,
};
use rdf_model::{HeapQuad, HeapTerm, Statement, StatementPattern};
use rdf_store::{ReadTransaction, WriteTransaction};

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A transaction for reading and writing statements in MongoDB.
///
/// # Examples
///
/// Mutate the store in a write transaction:
///
/// ```rust,compile_fail
/// let mut tx = store.write().await?;
///
/// tx.remove(old_quad.into()).await?;
/// tx.insert(new_quad.into()).await?;
///
/// tx.commit().await?;
/// ```
///
/// Access the store in a read-only transaction:
///
/// ```rust,compile_fail
/// let tx = store.read().await?;
///
/// tx.r#match(quad_pattern)
///     .for_each(|quad| async move {
///         eprintln!("{:?}", quad);
///     })
///     .await;
/// ```
///
/// # Flows
///
/// See: <https://www.mongodb.com/docs/manual/core/transactions/>
#[derive(Debug)]
pub struct MongoTransaction {
    #[debug(skip)]
    pub session: ClientSession,
    pub spo: Collection<Document>,
    pub graph: Collection<Document>,
    pub writable: bool,
    pub isolated: bool,
}

impl MongoTransaction {
    pub async fn begin(store: &MongoStore, writable: bool) -> Result<Self, MongoError> {
        let mut session = store
            .client
            .start_session()
            .with_options(SessionOptions::default()) // TODO
            .await?;

        let db = store.database();
        let spo = db.collection("rdf:spo");
        let graph = db.collection("rdf:g:default");

        let isolated = false; // TODO: when is this possible to enable?
        if isolated {
            session
            .start_transaction()
            //.with_options(Some(TransactionOptions))
            .await?;
        }

        Ok(Self {
            session,
            spo,
            graph,
            writable,
            isolated,
        })
    }

    pub fn is_writable(&self) -> bool {
        self.writable
    }
}

#[async_trait]
impl WriteTransaction for MongoTransaction {
    type Error = MongoError;
    type Statement = HeapQuad;

    async fn rollback(mut self) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(MongoError::ReadOnly);
        };

        if self.isolated {
            self.session.abort_transaction().await?;
        }

        Ok(())
    }

    async fn commit(mut self) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(MongoError::ReadOnly);
        };

        if self.isolated {
            self.session.commit_transaction().await?;
        }

        Ok(())
    }

    async fn insert(&mut self, statement: &Self::Statement) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(MongoError::ReadOnly);
        };

        if statement.has_context() {
            return Err(MongoError::Other); // TODO: named graphs not supported yet
        }

        let triple_doc: Document = MongoTriple::from(statement).into();

        let result = self
            .spo
            .insert_one(triple_doc)
            .session(&mut self.session)
            .await;
        match result {
            Ok(_) => {},
            Err(err) => {
                match *err.kind.clone() {
                    ErrorKind::Write(WriteFailure::WriteError(WriteError {
                        code: 11000, .. // E11000 duplicate key error
                    })) => {},
                    _ => return Err(err.into()),
                };
            },
        };

        let triple_doc: Document = MongoTripleId::from(statement).into();
        let result = self
            .graph
            .insert_one(triple_doc)
            .session(&mut self.session)
            .await;
        match result {
            Ok(_) => {},
            Err(err) => {
                match *err.kind.clone() {
                    ErrorKind::Write(WriteFailure::WriteError(WriteError {
                        code: 11000, .. // E11000 duplicate key error
                    })) => {},
                    _ => return Err(err.into()),
                };
            },
        };

        Ok(())
    }

    async fn remove(&mut self, statement: &Self::Statement) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(MongoError::ReadOnly);
        };

        if statement.has_context() {
            return Err(MongoError::Other); // TODO: named graphs not supported yet
        }

        let triple_doc: Document = MongoTripleId::from(statement).into();
        //std::eprintln!("{:?}", &triple_doc); // DEBUG

        self.graph
            .delete_one(triple_doc)
            .session(&mut self.session)
            .await?;

        Ok(())
    }
}

#[async_trait]
impl ReadTransaction for MongoTransaction {
    type Error = MongoError;
    type Statement = HeapQuad;
    type Term = HeapTerm;

    fn r#match(
        &self,
        _pattern: Option<impl StatementPattern<Term = Self::Term>>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> {
        stream::empty() // TODO
    }
}
