// This is free and unencumbered software released into the public domain.

pub trait Durable {
    fn is_durable(&self) -> bool;

    fn is_nondurable(&self) -> bool {
        !self.is_durable()
    }
}
