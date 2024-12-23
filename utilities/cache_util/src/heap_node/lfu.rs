use super::HeapNodeTrait;
use std::cmp::Ordering;

pub struct LFUHeapNode<H: HeapNodeTrait> {
    freq: i32,
    node: H,
}

impl<H: HeapNodeTrait> PartialEq for LFUHeapNode<H> {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq && self.node == other.node
    }
}
impl<H: HeapNodeTrait> Eq for LFUHeapNode<H> {}

impl<H: HeapNodeTrait> PartialOrd for LFUHeapNode<H> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<H: HeapNodeTrait> Ord for LFUHeapNode<H> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.freq.cmp(&other.freq) {
            Ordering::Equal => self.node.cmp(&other.node), // min heap
            ord => ord,
        }
    }
}

impl<H: HeapNodeTrait> HeapNodeTrait for LFUHeapNode<H> {
    type Key = H::Key; // pass key to inner node
    type Value = H::Value; // pass value to inner node

    fn new(key: Self::Key, value: Self::Value) -> Self {
        Self {
            freq: 1,
            node: HeapNodeTrait::new(key, value),
        }
    }

    fn key(&self) -> &Self::Key {
        self.node.key()
    }
    fn value(&self) -> &Self::Value {
        self.node.value()
    }
    fn set_value(&mut self, value: Self::Value) {
        self.node.set_value(value);
    }

    fn on_access(&mut self) {
        self.node.on_access();
        self.freq += 1;
    }
}
