use super::HeapNodeTrait;
use std::cmp::Ordering;

pub struct KeyAwareHeapNode<H>
where
    H: HeapNodeTrait<Key = ()>,
{
    key: i32,
    node: H,
}

impl<H> PartialEq for KeyAwareHeapNode<H>
where
    H: HeapNodeTrait<Key = ()>,
{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.node == other.node
    }
}
impl<H> Eq for KeyAwareHeapNode<H> where H: HeapNodeTrait<Key = ()> {}

impl<H> PartialOrd for KeyAwareHeapNode<H>
where
    H: HeapNodeTrait<Key = ()>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<H> Ord for KeyAwareHeapNode<H>
where
    H: HeapNodeTrait<Key = ()>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.node.cmp(&other.node) // ignore key
    }
}

impl<H> HeapNodeTrait for KeyAwareHeapNode<H>
where
    H: HeapNodeTrait<Key = ()>,
{
    type Key = i32;

    fn new(key: Self::Key) -> Self {
        Self {
            key,
            node: HeapNodeTrait::new(()),
        }
    }

    fn key(&self) -> &Self::Key {
        &self.key
    }

    fn on_access(&mut self) {
        self.node.on_access();
    }
}
