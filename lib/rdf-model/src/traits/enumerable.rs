// This is free and unencumbered software released into the public domain.

extern crate alloc;

use super::Countable;
use crate::Statement;
use alloc::boxed::Box;
use core::error::Error;

pub trait Enumerable:
    Countable + Iterator<Item = Result<Box<dyn Statement>, Box<dyn Error>>>
{
    fn grep(&self, pattern: &impl PartialEq<Box<dyn Statement>>) -> Self
    where
        Self: Sized;
}
