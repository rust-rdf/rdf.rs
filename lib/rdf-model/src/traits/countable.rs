// This is free and unencumbered software released into the public domain.

pub trait Countable {
    fn count(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.count() == 0
    }
}
