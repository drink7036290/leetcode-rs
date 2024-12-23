use super::{EvictionAsStoragePolicy, EvictionPolicy};
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
    H: HeapNodeTrait<Key = (), Value = ()>,
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
            self.pq.push(key, Reverse(HeapNodeTrait::new((), ())));
        }
    }

    fn evict(&mut self) -> Option<i32> {
        self.pq.pop().map(|(key, _)| key)
    }
}

impl<H> EvictionAsStoragePolicy for EvictionPolicyPQ<H>
where
    H: HeapNodeTrait<Key = (), Value = i32>,
{
    fn evict(&mut self) -> Option<i32> {
        self.pq.pop().map(|(key, _)| key)
    }

    fn get(&mut self, key: &i32) -> Option<i32> {
        let mut result = None;
        self.pq.change_priority_by(key, |p| {
            p.0.on_access();
            result = Some(*p.0.value());
        });

        result
    }

    fn put(&mut self, key: i32, value: i32) {
        if !self.pq.change_priority_by(&key, |p| {
            p.0.on_access();
        }) {
            self.pq.push(key, Reverse(HeapNodeTrait::new((), value)));
        }
    }

    fn len(&self) -> usize {
        self.pq.len()
    }

    fn is_empty(&self) -> bool {
        self.pq.is_empty()
    }
}
