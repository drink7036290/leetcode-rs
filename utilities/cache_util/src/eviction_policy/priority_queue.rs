use super::EvictionPolicy;
use crate::HeapNodeTrait;

use priority_queue::PriorityQueue;
use std::cmp::Reverse;

pub struct EvictionPolicyPQ<H>
where
    H: HeapNodeTrait<Key = ()>,
{
    pq: PriorityQueue<i32, Reverse<H>>,
}

impl<H> EvictionPolicyPQ<H>
where
    H: HeapNodeTrait<Key = ()>,
{
    pub fn new() -> Self {
        Self {
            pq: PriorityQueue::<i32, Reverse<H>>::new(),
        }
    }
}

impl<H> Default for EvictionPolicyPQ<H>
where
    H: HeapNodeTrait<Key = ()>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<H> EvictionPolicy for EvictionPolicyPQ<H>
where
    H: HeapNodeTrait<Key = ()>,
{
    fn on_get(&mut self, key: &i32) {
        self.pq.change_priority_by(key, |p| {
            p.0.on_access();
        });
    }

    fn on_put(&mut self, key: i32) {
        if !self.pq.change_priority_by(&key, |p| {
            p.0.on_access();
        }) {
            self.pq.push(key, Reverse(HeapNodeTrait::new(())));
        }
    }

    fn evict(&mut self) -> Option<i32> {
        self.pq.pop().map(|(key, _)| key)
    }
}