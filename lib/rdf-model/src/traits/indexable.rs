// This is free and unencumbered software released into the public domain.

pub trait Indexable {
    fn is_indexed(&self) -> bool;

    fn is_unindexed(&self) -> bool {
        !self.is_indexed()
    }
}

pub trait IndexableMut: Indexable {
    fn index(&mut self) -> Result<(), ()>;
}
