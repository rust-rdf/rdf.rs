// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use core::pin::Pin;
use futures::{Stream, StreamExt};
use tokio::runtime::Handle;

/// An iterator that blocks on a `Stream` using a Tokio runtime `Handle`.
///
/// This stores the stream as a pinned boxed trait object so the underlying
/// stream implementation doesn't need to implement `Unpin`.
pub struct StreamIter<I> {
    stream: Pin<Box<dyn Stream<Item = I> + Send>>,
    handle: Handle,
}

impl<I> StreamIter<I> {
    pub fn new<S>(stream: S, handle: Handle) -> Self
    where
        S: Stream<Item = I> + Send + 'static,
    {
        Self {
            stream: Box::pin(stream),
            handle,
        }
    }
}

impl<I> Iterator for StreamIter<I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        self.handle.block_on(self.stream.next())
    }
}
