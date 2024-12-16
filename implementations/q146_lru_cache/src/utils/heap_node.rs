use std::cmp::Ordering;
use std::time::SystemTime;

pub struct HeapNode {
    pub key: i32,
    pub val: i32,
    pub last_access: SystemTime,
}

impl HeapNode {
    pub fn new(key: i32, val: i32) -> Self {
        HeapNode {
            key,
            val,
            last_access: SystemTime::now(),
        }
    }
}

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.last_access == other.last_access
    }
}
impl Eq for HeapNode {}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.last_access.cmp(&other.last_access)
    }
}