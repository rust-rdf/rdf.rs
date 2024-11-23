// This is free and unencumbered software released into the public domain.

pub trait MaybeIndexed {
    fn is_indexed(&self) -> bool {
        false // by default
    }

    fn is_unindexed(&self) -> bool {
        !self.is_indexed()
    }
}
