// This is free and unencumbered software released into the public domain.

use super::SqliteError;
use alloc::boxed::Box;
use async_trait::async_trait;
use rdf_model::{HeapQuad, HeapTerm, Statement, Term, TermKind};
use rdf_store::Transaction;

pub struct SqliteTransaction<'conn> {
    pub(crate) tx: turso::transaction::Transaction<'conn>,
}

impl<'conn> SqliteTransaction<'conn> {
    async fn insert_triple(&mut self, s: u64, p: u64, o: u64) -> Result<bool, SqliteError> {
        self.tx
            .execute(
                "INSERT INTO rdf_triple (s, p, o) VALUES (?, ?, ?) ON CONFLICT DO NOTHING",
                (s, p, o),
            )
            .await
            .map(|count| count > 0)
    }

    #[allow(unused)]
    async fn insert_triple_num(
        &mut self,
        s: u64,
        p: u64,
        o_dt: Option<u64>,
        o_val: &str,
    ) -> Result<bool, SqliteError> {
        self.tx
            .execute(
                "INSERT INTO rdf_triple_num (s, p, o_dt, o_val) VALUES (?, ?, ?, ?) ON CONFLICT DO NOTHING",
                (s, p, o_dt.unwrap_or(0), o_val),
            )
            .await
            .map(|count| count > 0)
    }

    async fn insert_triple_str(
        &mut self,
        s: u64,
        p: u64,
        o_dt: Option<u64>,
        o_lang: Option<&str>,
        o_val: &str,
    ) -> Result<bool, SqliteError> {
        self.tx
            .execute(
                "INSERT INTO rdf_triple_str (s, p, o_dt, o_lang, o_val) VALUES (?, ?, ?, ?, ?) ON CONFLICT DO NOTHING",
                (s, p, o_dt.unwrap_or(0), o_lang.unwrap_or(""), o_val),
            )
            .await
            .map(|count| count > 0)
    }

    async fn intern_node(&mut self, term: &HeapTerm) -> Result<u64, SqliteError> {
        Ok(match term {
            HeapTerm::Iri(val) => self.intern_iri(val).await?,
            HeapTerm::BNode(val) => self.intern_bnode(val).await?,
            HeapTerm::Literal(_)
            | HeapTerm::LiteralWithDatatype(_, _)
            | HeapTerm::LiteralWithLanguage(_, _) => unreachable!(),
        })
    }

    async fn intern_iri(&mut self, val: &str) -> Result<u64, SqliteError> {
        let mut rows = self.tx
            .query(
                "INSERT INTO rdf_node (kind, val) VALUES (?, ?) ON CONFLICT (kind, val) DO UPDATE SET kind = excluded.kind RETURNING id",
                (0, val,),
            )
            .await?;
        let row = rows.next().await?.expect("should return a row");
        row.get::<u64>(0)
    }

    async fn intern_bnode(&mut self, val: &str) -> Result<u64, SqliteError> {
        let mut rows = self.tx
            .query(
                "INSERT INTO rdf_node (kind, val) VALUES (?, ?) ON CONFLICT (kind, val) DO UPDATE SET kind = excluded.kind RETURNING id",
                (1, val,),
            )
            .await?;
        let row = rows.next().await?.expect("should return a row");
        row.get::<u64>(0)
    }
}

#[async_trait]
impl<'conn> Transaction for SqliteTransaction<'conn> {
    type Error = SqliteError;
    type Statement = HeapQuad;

    async fn insert_statement(&mut self, statement: &Self::Statement) -> Result<(), Self::Error> {
        use HeapTerm::*;
        let _g = match statement.context() {
            Some(g) => Some(self.intern_node(g).await?),
            None => None,
        };
        let s = self.intern_node(statement.subject()).await?;
        let p = self.intern_node(statement.predicate()).await?;
        match statement.object() {
            Iri(_) | BNode(_) => {
                let o = self.intern_node(statement.object()).await?;
                self.insert_triple(s, p, o).await?;
            }
            Literal(o_val) => {
                self.insert_triple_str(s, p, None, None, o_val).await?;
            }
            LiteralWithLanguage(o_val, o_lang) => {
                self.insert_triple_str(s, p, None, Some(o_lang.as_str()), o_val)
                    .await?;
            }
            LiteralWithDatatype(o_val, o_dt) => {
                let o_dt = self.intern_iri(&o_dt).await?;
                self.insert_triple_str(s, p, Some(o_dt), None, o_val)
                    .await?; // TODO: numeric datatypes
            }
        }
        Ok(()) // TODO
    }

    async fn remove_statement(&mut self, _statement: &Self::Statement) -> Result<(), Self::Error> {
        Ok(()) // TODO
    }

    async fn commit(self) -> Result<(), Self::Error> {
        Ok(self.tx.commit().await?)
    }

    async fn rollback(self) -> Result<(), Self::Error> {
        Ok(self.tx.rollback().await?)
    }
}
