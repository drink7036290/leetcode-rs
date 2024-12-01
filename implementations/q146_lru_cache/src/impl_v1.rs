use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::time::SystemTime;

use crate::heap_node::HeapNode;

pub struct LRUCache {
    pq: PriorityQueue<i32, Reverse<HeapNode>>,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
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
                    p.0.last_access = SystemTime::now();

                    p.0.val = value;
                });
            }
            None => {
                if self.pq.len() == self.capacity {
                    self.pq.pop();
                }

                self.pq.push(key, Reverse(HeapNode::new(key, value)));
            }
        }
    }
}

/*
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
