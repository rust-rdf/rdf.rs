// This is free and unencumbered software released into the public domain.

pub trait MaybeMutable {
    fn is_mutable(&self) -> bool {
        false // by default
    }

    fn is_immutable(&self) -> bool {
        !self.is_mutable()
    }
}
