use priority_queue::PriorityQueue;
use std::cmp::{Ordering, Reverse};
use std::time::SystemTime;

#[derive(Clone, Debug)]
struct HeapNode {
    val: i32,
    freq: i32,
    last_access: SystemTime,
}

impl HeapNode {
    fn new(val: i32) -> Self {
        HeapNode {
            val,
            freq: 1,
            last_access: SystemTime::now(),
        }
    }
}

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq && self.last_access == other.last_access
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
        match self.freq.cmp(&other.freq) {
            Ordering::Equal => self.last_access.cmp(&other.last_access),
            ord => ord,
        }
    }
}

pub struct LFUCache {
    pq: PriorityQueue<i32, Reverse<HeapNode>>,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            pq: PriorityQueue::new(),
            capacity: capacity as usize,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        match self.pq.get(&key) {
            Some((_, rev_node)) => {
                let val = rev_node.0.val;

                self.pq.change_priority_by(&key, |p| {
                    p.0.freq += 1;
                    p.0.last_access = SystemTime::now();
                });

                val
            }
            None => -1,
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        match self.pq.get(&key) {
            Some((_, _)) => {
                self.pq.change_priority_by(&key, |p| {
                    p.0.freq += 1;
                    p.0.last_access = SystemTime::now();

                    p.0.val = value;
                });
            }
            None => {
                if self.pq.len() == self.capacity {
                    self.pq.pop();
                }

                self.pq.push(key, Reverse(HeapNode::new(value)));
            }
        }
    }
}

/*
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
