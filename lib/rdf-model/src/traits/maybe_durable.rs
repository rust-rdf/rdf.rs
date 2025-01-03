// This is free and unencumbered software released into the public domain.

pub trait MaybeDurable {
    fn is_durable(&self) -> bool {
        false // by default
    }

    fn is_nondurable(&self) -> bool {
        !self.is_durable()
    }
}
