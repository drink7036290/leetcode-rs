use super::{EvictionAsStoragePolicy, EvictionPolicy};
use crate::HeapNodeTrait;

use std::cmp::Ordering;
use std::collections::HashMap;

pub struct EvictionPolicyVHM<H> {
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

    fn get_node(&self, index: usize) -> Option<&H> {
        match index.cmp(&self.arr.len()) {
            Ordering::Less => Some(&self.arr[index]),
            _ => None,
        }
    }

    fn get_parent(&self, index: usize) -> Option<(usize, &H)> {
        match index {
            0 => None,
            _ => {
                let parent_index = (index - 1) >> 1;
                self.get_node(parent_index).map(|node| (parent_index, node))
            }
        }
    }

    fn get_left_child(&self, index: usize) -> Option<(usize, &H)> {
        let left_child_index = (index << 1) + 1;
        self.get_node(left_child_index)
            .map(|node| (left_child_index, node))
    }

    fn get_right_child(&self, index: usize) -> Option<(usize, &H)> {
        let right_child_index = (index << 1) + 2;
        self.get_node(right_child_index)
            .map(|node| (right_child_index, node))
    }

    fn swap_nodes_with_key(&mut self, key1: i32, index1: usize, key2: i32, index2: usize) {
        match self.map.get_mut(&key1) {
            Some(v) => *v = index2,
            None => panic!("key {} not found in map", key1),
        }

        match self.map.get_mut(&key2) {
            Some(v) => *v = index1,
            None => panic!("key {} not found in map", key2),
        }

        self.arr.swap(index1, index2);
    }

    fn swap_nodes(&mut self, index1: usize, index2: usize) {
        let key1 = match self.get_node(index1) {
            Some(v) => v.key(),
            None => panic!("Could not find node with vec index {} ", index1),
        };

        let key2 = match self.get_node(index2) {
            Some(v) => v.key(),
            None => panic!("Could not find node with vec index {} ", index2),
        };

        self.swap_nodes_with_key(*key1, index1, *key2, index2);
    }

    fn sift_up(&mut self, index: usize) {
        let mut index = index;

        while let Some((parent_index, parent_node)) = self.get_parent(index) {
            let node = match self.get_node(index) {
                Some(v) => v,
                None => break,
            };

            // already in order
            if *node >= *parent_node {
                break;
            }

            // swap with parent
            self.swap_nodes_with_key(*node.key(), index, *parent_node.key(), parent_index);
            index = parent_index;
        }
    }

    fn sift_down(&mut self, index: usize) {
        let mut index = index;

        while let Some((left_child_index, left_child_node)) = self.get_left_child(index) {
            let node = match self.get_node(index) {
                Some(v) => v,
                None => break,
            };

            // right child exists
            if let Some((right_child_index, right_child_node)) = self.get_right_child(index) {
                // right child is smaller
                if *left_child_node >= *right_child_node {
                    // already in order
                    if *node <= *right_child_node {
                        break;
                    }

                    // swap with right child
                    self.swap_nodes_with_key(
                        *node.key(),
                        index,
                        *right_child_node.key(),
                        right_child_index,
                    );
                    index = right_child_index;

                    continue;
                }
            }

            // already in order
            if *node <= *left_child_node {
                break;
            }

            // swap with left child
            self.swap_nodes_with_key(*node.key(), index, *left_child_node.key(), left_child_index);
            index = left_child_index;
        }
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
        if let Some(index) = self.map.get(key) {
            if index.cmp(&self.arr.len()).is_lt() {
                self.arr[*index].on_access();
                self.sift_down(*index);
            }
        }
    }

    fn on_put(&mut self, key: i32) {
        if let Some(index) = self.map.get(&key) {
            self.sift_down(*index);
        } else {
            self.arr.push(HeapNodeTrait::new(key, ()));
            self.map.insert(key, self.arr.len() - 1);
            self.sift_up(self.arr.len() - 1);
        }
    }

    fn evict(&mut self) -> Option<i32> {
        if self.arr.is_empty() {
            return None;
        }

        let last_index = self.arr.len() - 1;

        self.swap_nodes(0, last_index);
        self.map.remove(self.arr[last_index].key());
        let result = self.arr.pop().map(|node| *node.key());
        self.sift_down(0);

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

        self.swap_nodes(0, last_index);
        self.map.remove(self.arr[last_index].key());
        let result = self.arr.pop().map(|node| *node.key());
        self.sift_down(0);

        result
    }

    fn get(&mut self, key: &i32) -> Option<i32> {
        if let Some(index) = self.map.get(key).cloned() {
            if index.cmp(&self.arr.len()).is_lt() {
                let val = *self.arr[index].value();
                self.arr[index].on_access();
                self.sift_down(index);

                return Some(val);
            }
        }

        None
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(index) = self.map.get(&key).cloned() {
            self.arr[index].set_value(value);
            self.sift_down(index);
        } else {
            self.arr.push(HeapNodeTrait::new(key, value));
            self.map.insert(key, self.arr.len() - 1);
            self.sift_up(self.arr.len() - 1);
        }
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
