use super::{EvictionAsStoragePolicy, EvictionPolicy};
use crate::HeapNodeTrait;

use std::collections::HashMap;
use std::mem::swap;

pub struct EvictionPolicyVHM<H>
where
    H: HeapNodeTrait<Key = i32>,
{
    map: HashMap<i32, usize>, // key -> vec's index
    arr: Vec<H>,
}

impl<H> EvictionPolicyVHM<H>
where
    H: HeapNodeTrait<Key = i32>,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            arr: Vec::new(),
        }
    }

    fn get_left_child(&self, mut index: usize) -> Option<usize> {
        index = (index << 1) + 1;
        if index < self.arr.len() {
            return Some(index);
        }

        None
    }

    fn swap_nodes_with_key(&mut self, key1: &i32, key2: &i32) {
        if let [Some(index1), Some(index2)] = self.map.get_many_mut([key1, key2]) {
            self.arr.swap(*index1, *index2);
            swap(index1, index2);
        }
    }

    fn swap_nodes(&mut self, index1: usize, index2: usize) {
        debug_assert!(index1 < self.arr.len());
        debug_assert!(index2 < self.arr.len());

        if index1 == index2 {
            return;
        }

        let (key1, key2) = (*self.arr[index1].key(), *self.arr[index2].key());
        self.swap_nodes_with_key(&key1, &key2);
    }

    fn sift_up(&mut self, mut index: usize) {
        debug_assert!(index < self.arr.len());

        while index > 0 {
            let parent_index = (index - 1) >> 1;

            let (node, parent_node) = (&self.arr[index], &self.arr[parent_index]);

            // already in order
            if *node >= *parent_node {
                break;
            }

            let (key, parent_key) = (*node.key(), *parent_node.key());
            self.swap_nodes_with_key(&key, &parent_key);

            index = parent_index;
        }
    }

    fn sift_down(&mut self, mut index: usize) {
        debug_assert!(index < self.arr.len());

        while let Some(left_child_index) = self.get_left_child(index) {
            let node = &self.arr[index];

            let (next_index, next_node) = self.pick_smaller_child(left_child_index);

            // already in order
            if *node <= *next_node {
                break;
            }

            let (key, next_key) = (*node.key(), *next_node.key());
            self.swap_nodes_with_key(&key, &next_key);

            index = next_index;
        }
    }

    fn pick_smaller_child(&self, left_child_index: usize) -> (usize, &H) {
        let left_child_node = &self.arr[left_child_index];

        let right_child_index = left_child_index + 1;
        if right_child_index < self.arr.len() {
            let right_child_node = &self.arr[right_child_index];

            // right child is strictly smaller
            if *left_child_node > *right_child_node {
                return (right_child_index, right_child_node);
            }
        }

        (left_child_index, left_child_node)
    }
}

impl<H> Default for EvictionPolicyVHM<H>
where
    H: HeapNodeTrait<Key = i32>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<H> EvictionPolicy for EvictionPolicyVHM<H>
where
    H: HeapNodeTrait<Key = i32, Value = ()>,
{
    fn on_get(&mut self, key: &i32) {
        if let Some(index) = self.map.get(key).cloned() {
            self.arr[index].on_access();
            self.sift_down(index);
        }
    }

    fn on_put(&mut self, key: i32) {
        if let Some(index) = self.map.get(&key) {
            self.sift_down(*index);
        } else {
            self.arr.push(HeapNodeTrait::new(key, ()));

            let index = self.arr.len() - 1;
            self.map.insert(key, index);
            self.sift_up(index);
        }
    }

    fn evict(&mut self) -> Option<i32> {
        /*         if let Some(node) = self.arr.swap_remove_back(0) {
            self.map.remove(node.key());

            if self.arr.is_empty() {
                return None;
            }

            let key = *self.arr[0].key();
            if let Some(index) = self.map.get_mut(&key) {
                *index = 0;
            }

            self.sift_down(0);

            return Some(*node.key());
        }

        None */

        if self.arr.is_empty() {
            return None;
        }

        let last_index = self.arr.len() - 1;

        if last_index > 0 {
            self.swap_nodes(0, last_index);
        }

        self.map.remove(self.arr[last_index].key());
        let result = self.arr.pop().map(|node| *node.key());

        if last_index > 0 {
            self.sift_down(0);
        }

        result
    }
}

impl<H> EvictionAsStoragePolicy for EvictionPolicyVHM<H>
where
    H: HeapNodeTrait<Key = i32, Value = i32>,
{
    fn evict(&mut self) -> Option<i32> {
        if self.arr.is_empty() {
            return None;
        }

        let last_index = self.arr.len() - 1;

        if last_index > 0 {
            self.swap_nodes(0, last_index);
        }

        self.map.remove(self.arr[last_index].key());
        let result = self.arr.pop().map(|node| *node.key());

        if last_index > 0 {
            self.sift_down(0);
        }

        result
    }

    fn get(&mut self, key: &i32) -> Option<i32> {
        self.map.get(key).cloned().map(|index| {
            let node = &mut self.arr[index];
            let val = *node.value();

            node.on_access();
            self.sift_down(index);

            val
        })
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(index) = self.map.get(&key).cloned() {
            self.arr[index].set_value(value);
            self.sift_down(index);
        } else {
            self.arr.push(HeapNodeTrait::new(key, value));

            let index = self.arr.len() - 1;
            self.map.insert(key, index);
            self.sift_up(index);
        }
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
