// This is free and unencumbered software released into the public domain.

pub trait Mutable {
    fn is_mutable(&self) -> bool;

    fn is_immutable(&self) -> bool {
        !self.is_mutable()
    }
}