use crate::FreqAwareHeapNode;
use std::collections::HashMap;

use std::cmp::Ordering;
use std::time::SystemTime;

pub struct LFUCache {
    arr: Vec<FreqAwareHeapNode>,
    map: HashMap<i32, usize>, // key -> vec's index
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    fn get_node(&self, index: usize) -> Option<&FreqAwareHeapNode> {
        match index.cmp(&self.arr.len()) {
            Ordering::Less => Some(&self.arr[index]),
            _ => None,
        }
    }

    fn get_node_mut(&mut self, index: usize) -> Option<&mut FreqAwareHeapNode> {
        match index.cmp(&self.arr.len()) {
            Ordering::Less => Some(&mut self.arr[index]),
            _ => None,
        }
    }

    fn get_parent(&self, index: usize) -> Option<(usize, &FreqAwareHeapNode)> {
        match index {
            0 => None,
            _ => {
                let parent_index = (index - 1) >> 1;
                self.get_node(parent_index).map(|node| (parent_index, node))
            }
        }
    }

    fn get_left_child(&self, index: usize) -> Option<(usize, &FreqAwareHeapNode)> {
        let left_child_index = (index << 1) + 1;
        self.get_node(left_child_index)
            .map(|node| (left_child_index, node))
    }

    fn get_right_child(&self, index: usize) -> Option<(usize, &FreqAwareHeapNode)> {
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
            Some(v) => v.node.key,
            None => panic!("Could not find node with vec index {} ", index1),
        };

        let key2 = match self.get_node(index2) {
            Some(v) => v.node.key,
            None => panic!("Could not find node with vec index {} ", index2),
        };

        self.swap_nodes_with_key(key1, index1, key2, index2);
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
            self.swap_nodes_with_key(node.node.key, index, parent_node.node.key, parent_index);
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
                        node.node.key,
                        index,
                        right_child_node.node.key,
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
            self.swap_nodes_with_key(
                node.node.key,
                index,
                left_child_node.node.key,
                left_child_index,
            );
            index = left_child_index;
        }
    }

    pub fn new(capacity: i32) -> Self {
        Self {
            arr: Vec::with_capacity(capacity as usize),
            map: HashMap::with_capacity(capacity as usize),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let index = match self.map.get(&key) {
            Some(v) => *v,
            None => return -1,
        };

        let node = match self.get_node_mut(index) {
            Some(v) => v,
            None => panic!(
                "vec index {} found with key {} but node is None",
                index, key
            ),
        };

        node.freq += 1;
        node.node.last_access = SystemTime::now();

        let val = node.node.val;
        self.sift_down(index);
        val
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let index = match self.map.get(&key) {
            Some(v) => *v,
            None => {
                if self.arr.len() == self.arr.capacity() {
                    let last_index = self.arr.len() - 1;

                    self.swap_nodes(0, last_index);
                    self.map.remove(&self.arr[last_index].node.key);
                    self.arr.pop();
                    self.sift_down(0);
                }

                self.arr.push(FreqAwareHeapNode::new(key, value));
                self.map.insert(key, self.arr.len() - 1);
                self.sift_up(self.arr.len() - 1);

                return;
            }
        };

        let node = match self.get_node_mut(index) {
            Some(v) => v,
            None => panic!(
                "vec index {} found with key {} but node is None",
                index, key
            ),
        };

        node.node.val = value;

        node.freq += 1;
        node.node.last_access = SystemTime::now();

        self.sift_down(index);
    }
}

/*
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
