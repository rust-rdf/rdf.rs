// This is free and unencumbered software released into the public domain.

use super::{ReadTransaction, WriteTransaction};
use core::fmt::Debug;

/// A store of statements that supports R/O and R/W transactions.
///
/// # Semantics for implementors
///
/// - The `Store` trait provides factories for `ReadTransaction` and
///   `WriteTransaction` instances. Implementations **should** document the
///   isolation guarantees provided by transactions (snapshot vs. non-snapshot
///   reads, write visibility, etc.).
/// - Creating a transaction must not mutate global store state. Resource
///   allocation performed while creating a transaction (locks, buffers, OS
///   handles) should be released promptly if the creation future is dropped.
/// - Implementations should avoid lock ordering inversions between transaction
///   creation and transaction method implementations. Prefer a consistent
///   global order for acquiring internal locks to reduce deadlock risk.
/// - The cancel-safety guarantees of the returned transaction objects are
///   governed by `ReadTransaction` and `WriteTransaction` semantics;
///   implementors should ensure those semantics hold for the concrete
///   transaction types they return.
pub trait Store {
    type Error: Debug + Send;
    type Read: ReadTransaction<Error = Self::Error> + Send;
    type Write: WriteTransaction<Error = Self::Error> + Send;

    /// Begins a read-only (R/O) transaction.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. Starting a read transaction is an
    /// observation-only operation that should not mutate store state. Canceling
    /// or dropping the returned future before it resolves must not leave the
    /// store in a partially-initialized state or leak resources. Implementations
    /// that allocate temporary resources when creating a transaction should
    /// release them promptly if the future is dropped.
    fn read(&mut self) -> impl Future<Output = Result<Self::Read, Self::Error>> + Send;

    /// Begins a read-write (R/W) transaction.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe in the sense that canceling the returned
    /// future before it resolves must not apply any mutations to the store. In
    /// particular, implementations should ensure that partially-started write
    /// transactions do not leave behind resources or locks. Once the future
    /// completes with a `Write` transaction, the usual `WriteTransaction` cancel
    /// safety rules apply: callers are responsible for driving `commit` to
    /// completion to ensure changes are applied atomically (see
    /// `WriteTransaction::commit`).
    fn write(&mut self) -> impl Future<Output = Result<Self::Write, Self::Error>> + Send;
}
