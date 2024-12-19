use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::time::SystemTime;

use crate::FreqAwareHeapNode;

pub struct LFUCache {
    pq: PriorityQueue<i32, Reverse<FreqAwareHeapNode>>,
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
                let val = rev_node.0.node.val;

                self.pq.change_priority_by(&key, |p| {
                    p.0.freq += 1;
                    p.0.node.last_access = SystemTime::now();
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
                    p.0.node.last_access = SystemTime::now();

                    p.0.node.val = value;
                });
            }
            None => {
                if self.pq.len() == self.capacity {
                    self.pq.pop();
                }

                self.pq
                    .push(key, Reverse(FreqAwareHeapNode::new(key, value)));
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
