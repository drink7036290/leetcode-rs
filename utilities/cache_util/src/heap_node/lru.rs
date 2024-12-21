use super::HeapNodeTrait;

use std::cmp::Ordering;
use std::time::SystemTime;

pub struct LRUHeapNode {
    last_access: SystemTime,
}

impl Default for LRUHeapNode {
    fn default() -> Self {
        Self::new(())
    }
}

impl PartialEq for LRUHeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.last_access == other.last_access
    }
}
impl Eq for LRUHeapNode {}

impl PartialOrd for LRUHeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for LRUHeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.last_access.cmp(&other.last_access)
    }
}

impl HeapNodeTrait for LRUHeapNode {
    type Key = ();

    fn new(_key: Self::Key) -> Self {
        Self {
            last_access: SystemTime::now(),
        }
    }
    fn key(&self) -> &() {
        &()
    }

    fn on_access(&mut self) {
        self.last_access = SystemTime::now();
    }
}
