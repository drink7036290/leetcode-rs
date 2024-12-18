use cache_util::HeapNode;
use std::cmp::Ordering;

pub struct FreqAwareHeapNode {
    pub freq: i32,
    pub node: HeapNode,
}

impl FreqAwareHeapNode {
    pub fn new(key: i32, val: i32) -> Self {
        FreqAwareHeapNode {
            freq: 1,
            node: HeapNode::new(key, val),
        }
    }
}

impl PartialEq for FreqAwareHeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq && self.node == other.node
    }
}
impl Eq for FreqAwareHeapNode {}

impl PartialOrd for FreqAwareHeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for FreqAwareHeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.freq.cmp(&other.freq) {
            Ordering::Equal => self.node.cmp(&other.node), // min heap
            ord => ord,
        }
    }
}
