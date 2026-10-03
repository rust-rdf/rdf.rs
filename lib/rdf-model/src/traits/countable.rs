// This is free and unencumbered software released into the public domain.

pub use dogma::traits::Countable;

/// Optional collection counts, available without `alloc` or `std`.
///
/// Since Dogma 0.3, use [`maybe_count()`](MaybeCountable::maybe_count),
/// [`maybe_is_empty()`](MaybeCountable::maybe_is_empty), and
/// [`maybe_is_nonempty()`](MaybeCountable::maybe_is_nonempty) instead of
/// `count()`, `is_empty()`, and `is_nonempty()`. The [`Countable`] methods
/// retain their original names.
pub use dogma::traits::MaybeCountable;
