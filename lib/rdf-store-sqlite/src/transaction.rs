// This is free and unencumbered software released into the public domain.

use super::{NodeId, SqliteError};
use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use async_stream::stream;
use async_trait::async_trait;
use futures::{Stream, stream::select};
use rdf_model::{
    Datatype, HeapQuad, HeapTerm, HeapTriple, Statement, StatementPattern, Term, TermKind,
};
use rdf_store::Transaction;

type Language = String; // TODO

#[derive(Debug)]
pub struct SqliteTransaction<'conn> {
    pub(crate) tx: turso::transaction::Transaction<'conn>,
}

#[async_trait]
impl<'conn> Transaction for SqliteTransaction<'conn> {
    type Error = SqliteError;
    type Term = HeapTerm;
    type Statement = HeapQuad;

    async fn rollback(self) -> Result<(), Self::Error> {
        Ok(self.tx.rollback().await?)
    }

    async fn commit(self) -> Result<(), Self::Error> {
        Ok(self.tx.commit().await?)
    }

    async fn insert_statement(&mut self, statement: &Self::Statement) -> Result<(), Self::Error> {
        use HeapTerm::*;
        let _g = match statement.context() {
            Some(g) => Some(self.intern_node(g).await?), // TODO
            None => None,
        };
        let s = self.intern_node(statement.subject()).await?;
        let p = self.intern_node(statement.predicate()).await?;
        match statement.object() {
            Iri(_) | BNode(_) => {
                let o = self.intern_node(statement.object()).await?;
                self.insert_triple(s, p, o).await?;
            }
            String(o_val) => {
                self.insert_triple_str(s, p, None, None, o_val).await?;
            }
            TaggedString(o_val, o_lang, _) => {
                self.insert_triple_str(s, p, None, Some(o_lang.as_str()), o_val)
                    .await?;
            }
            TypedValue(value) => {
                let o_dt = self
                    .intern_iri(value.r#type().iri_string().as_ref())
                    .await?;
                let o_val = value.to_string();
                self.insert_triple_str(s, p, Some(o_dt), None, &o_val)
                    .await?; // FIXME
            }
            TypedLiteral(o_val, o_dt) => {
                let o_dt = self.intern_iri(o_dt.iri_string().as_ref()).await?;
                self.insert_triple_str(s, p, Some(o_dt), None, o_val)
                    .await?;
            }
        }
        Ok(()) // TODO
    }

    async fn remove_statement(&mut self, _statement: &Self::Statement) -> Result<(), Self::Error> {
        Ok(()) // TODO
    }

    fn match_statements(
        &self,
        pattern: impl StatementPattern<Term = Self::Term>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> {
        let pattern = pattern.to_quad_pattern();
        let stream1 = self.match_triples(pattern.clone());
        let stream2 = self.match_triples_str(pattern.clone());
        let stream3 = self.match_triples_num(pattern.clone());
        select(stream1, select(stream2, stream3))
    }
}

impl<'conn> SqliteTransaction<'conn> {
    async fn insert_triple(
        &mut self,
        s: NodeId,
        p: NodeId,
        o: NodeId,
    ) -> Result<bool, SqliteError> {
        self.tx
            .execute(
                r#"
                INSERT INTO rdf_triple (s, p, o) VALUES (?, ?, ?)
                  ON CONFLICT DO NOTHING
                "#,
                (s, p, o),
            )
            .await
            .map(|count| count > 0)
    }

    #[allow(unused)]
    async fn insert_triple_num(
        &mut self,
        s: NodeId,
        p: NodeId,
        o_dt: Option<NodeId>,
        o_val: &str,
    ) -> Result<bool, SqliteError> {
        self.tx
            .execute(
                r#"
                INSERT INTO rdf_triple_num (s, p, o_dt, o_val) VALUES (?, ?, ?, ?)
                  ON CONFLICT DO NOTHING
                "#,
                (s, p, o_dt.unwrap_or(0), o_val),
            )
            .await
            .map(|count| count > 0)
    }

    async fn insert_triple_str(
        &mut self,
        s: NodeId,
        p: NodeId,
        o_dt: Option<NodeId>,
        o_lang: Option<&str>,
        o_val: &str,
    ) -> Result<bool, SqliteError> {
        self.tx
            .execute(
                r#"
                INSERT INTO rdf_triple_str (s, p, o_dt, o_lang, o_val) VALUES (?, ?, ?, ?, ?)
                  ON CONFLICT DO NOTHING
                "#,
                (s, p, o_dt.unwrap_or(0), o_lang.unwrap_or(""), o_val),
            )
            .await
            .map(|count| count > 0)
    }

    async fn intern_node(&mut self, term: &HeapTerm) -> Result<NodeId, SqliteError> {
        Ok(match term {
            HeapTerm::Iri(val) => self.intern_iri(val).await?,
            HeapTerm::BNode(val) => self.intern_bnode(val).await?,
            HeapTerm::String(_)
            | HeapTerm::TaggedString(_, _, _)
            | HeapTerm::TypedValue(_)
            | HeapTerm::TypedLiteral(_, _) => unreachable!(),
        })
    }

    async fn intern_iri(&mut self, val: &str) -> Result<NodeId, SqliteError> {
        let mut rows = self
            .tx
            .query(
                r#"
                INSERT INTO rdf_node (kind, val) VALUES (?, ?)
                  ON CONFLICT (kind, val) DO
                    UPDATE SET kind = excluded.kind RETURNING id
                "#,
                (0, val),
            )
            .await?;
        let row = rows.next().await?.expect("should return a row");
        row.get::<NodeId>(0)
    }

    async fn intern_bnode(&mut self, val: &str) -> Result<NodeId, SqliteError> {
        let mut rows = self
            .tx
            .query(
                r#"
                INSERT INTO rdf_node (kind, val) VALUES (?, ?)
                  ON CONFLICT (kind, val) DO
                    UPDATE SET kind = excluded.kind RETURNING id
                "#,
                (1, val),
            )
            .await?;
        let row = rows.next().await?.expect("should return a row");
        row.get::<NodeId>(0)
    }

    fn match_triples(
        &self,
        pattern: impl StatementPattern,
    ) -> impl Stream<Item = Result<HeapQuad, SqliteError>> {
        stream! {
            let mut rows = self.tx.query(r#"
                SELECT ns.val AS s, np.val AS p, no.val AS o
                  FROM rdf_triple t
                  JOIN rdf_node ns ON t.s = ns.id
                  JOIN rdf_node np ON t.p = np.id
                  JOIN rdf_node no ON t.o = no.id
            "#, ()).await?; // TODO: use the pattern

            while let Some(row) = rows.next().await? {
                let s = HeapTerm::iri(row.get::<String>(0)?);
                let p = HeapTerm::iri(row.get::<String>(1)?);
                let o = HeapTerm::iri(row.get::<String>(2)?);
                let g: Option<HeapTerm> = None;
                if pattern.matches(&s, &p, &o, g) {
                    yield Ok(HeapQuad::new(s, p, o, None));
                }
            }
        }
    }

    fn match_triples_str(
        &self,
        pattern: impl StatementPattern,
    ) -> impl Stream<Item = Result<HeapQuad, SqliteError>> {
        stream! {
            let mut rows = self.tx.query(r#"
                SELECT ns.val AS s, np.val AS p, nd.val AS o_dt, nullif(t.o_lang, ''), t.o_val
                  FROM rdf_triple_str t
                  LEFT JOIN rdf_node nd ON t.o_dt = nd.id
                  JOIN rdf_node ns ON t.s = ns.id
                  JOIN rdf_node np ON t.p = np.id
            "#, ()).await?; // TODO: use the pattern

            while let Some(row) = rows.next().await? {
                let s = HeapTerm::iri(row.get::<String>(0)?); // FIXME
                let p = HeapTerm::iri(row.get::<String>(1)?);
                let o_dt = row.get::<Option<String>>(2)?.map(Datatype::from_iri);
                let o_lang = row.get::<Option<Language>>(3)?;
                let o_val = row.get::<String>(4)?;
                let o = HeapTerm::from((o_val, o_dt, o_lang));
                let g: Option<HeapTerm> = None;
                if pattern.matches(&s, &p, &o, g) {
                    yield Ok(HeapQuad::new(s, p, o, None));
                }
            }
        }
    }

    fn match_triples_num(
        &self,
        pattern: impl StatementPattern,
    ) -> impl Stream<Item = Result<HeapQuad, SqliteError>> {
        stream! {
            let mut rows = self.tx.query(r#"
                SELECT ns.val AS s, np.val AS p, nd.val AS o_dt, t.o_val
                  FROM rdf_triple_num t
                  LEFT JOIN rdf_node nd ON t.o_dt = nd.id
                  JOIN rdf_node ns ON t.s = ns.id
                  JOIN rdf_node np ON t.p = np.id
            "#, ()).await?; // TODO: use the pattern

            while let Some(row) = rows.next().await? {
                let s = HeapTerm::iri(row.get::<String>(0)?); // FIXME
                let p = HeapTerm::iri(row.get::<String>(1)?);
                let o_dt = row.get::<Option<String>>(2)?.map(Datatype::from_iri);
                let o_val = row.get::<String>(3)?;
                let o = HeapTerm::from((o_val, o_dt, None));
                let g: Option<HeapTerm> = None;
                if pattern.matches(&s, &p, &o, g) {
                    yield Ok(HeapQuad::new(s, p, o, None));
                }
            }
        }
    }
}
