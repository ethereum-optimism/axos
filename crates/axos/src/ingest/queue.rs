//! Internal queue for ingesting blocks.

#[cfg(feature = "alloc")]
use alloc::collections::VecDeque;

use axos_primitives::BlockWithTransactions;

/// InnerQueue wraps a queue of [BlockWithTransactions].
#[derive(Debug, Default)]
pub struct InnerQueue(
    #[cfg(feature = "alloc")] pub VecDeque<BlockWithTransactions>,
    #[cfg(not(feature = "alloc"))] pub &'static mut VecDeque<BlockWithTransactions>,
);

impl Iterator for InnerQueue {
    type Item = BlockWithTransactions;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl InnerQueue {
    /// Return an iterator over the queue.
    pub fn iter(&self) -> impl Iterator<Item = &BlockWithTransactions> {
        self.0.iter()
    }

    /// Push a block onto the queue.
    pub fn push(&mut self, block: BlockWithTransactions) {
        self.0.push_back(block);
    }

    /// Returns the length of the queue.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Pops the front of the queue.
    pub fn pop_front(&mut self) -> Option<BlockWithTransactions> {
        self.0.pop_front()
    }

    /// Push to the back of the queue.
    pub fn push_back(&mut self, block: BlockWithTransactions) {
        self.0.push_back(block);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_queue() {
        let mut queue = InnerQueue::default();
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());

        queue.push(BlockWithTransactions::default());
        assert_eq!(queue.len(), 1);
        assert!(!queue.is_empty());

        queue.pop_front();
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
    }
}
