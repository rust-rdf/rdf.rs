// This is free and unencumbered software released into the public domain.

use futures::{Stream, StreamExt};
use tokio::runtime::Handle;

pub struct StreamIter<S> {
    stream: S,
    handle: Handle,
}

impl<S, I> StreamIter<S>
where
    S: Stream<Item = I> + Unpin,
{
    pub fn new(stream: S, handle: Handle) -> Self {
        Self { stream, handle }
    }
}

impl<S, I> Iterator for StreamIter<S>
where
    S: Stream<Item = I> + Unpin,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        self.handle.block_on(self.stream.next())
    }
}
