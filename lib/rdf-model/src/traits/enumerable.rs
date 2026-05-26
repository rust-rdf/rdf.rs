// This is free and unencumbered software released into the public domain.

use super::Countable;
use crate::Statement;
use alloc::boxed::Box;
use core::error::Error;

pub trait Enumerable: Countable + Iterator<Item = Result<Self::Statement, Box<dyn Error>>> {
    type Statement: Statement;
}
