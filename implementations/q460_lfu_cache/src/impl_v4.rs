use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

// Intrusive xor doubly-linked list which uses less memory than a regular doubly linked list
use intrusive_collections::intrusive_adapter;
use intrusive_collections::{LinkedList, LinkedListLink};

#[derive(Debug)]
struct Node {
    key: i32,
    val: Cell<i32>,
    freq: Cell<usize>,

    link: LinkedListLink,
}

intrusive_adapter!(NodeAdapter = Rc<Node>: Node { link: LinkedListLink });

impl Node {
    pub fn new(key: i32, val: i32) -> Self {
        Node {
            key,
            val: Cell::new(val),
            freq: Cell::new(1),
            link: LinkedListLink::default(),
        }
    }
}

pub struct LFUCache {
    map: HashMap<i32, Rc<Node>>,                       // key -> node
    freq_map: HashMap<usize, LinkedList<NodeAdapter>>, // freq -> list of nodes, ordered by last access time
    min_freq: usize,
    capacity: usize, // as HashMap's capacity() could be auto-resized
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::with_capacity(capacity as usize),
            freq_map: HashMap::with_capacity(capacity as usize),
            min_freq: 1,
            capacity: capacity as usize,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        // trick: cloned() to avoid multiple mutable self, also cloned Rc is cheap
        if let Some(node_rc) = self.map.get(&key).cloned() {
            let val = node_rc.val.get();
            self.update(node_rc);
            val
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        // trick: cloned() to avoid multiple mutable self, also cloned Rc is cheap
        if let Some(node_rc) = self.map.get(&key).cloned() {
            node_rc.val.set(value);
            self.update(node_rc);
        } else {
            if self.map.len() == self.capacity {
                if let Some(freq_list) = self.freq_map.get_mut(&self.min_freq) {
                    if let Some(node_rc) = freq_list.pop_front() {
                        self.map.remove(&node_rc.key);

                        if freq_list.is_empty() {
                            self.freq_map.remove(&self.min_freq);
                        }
                    }
                }
            }

            let node_rc = Rc::new(Node::new(key, value));
            self.map.insert(key, node_rc.clone());
            self.min_freq = 1;

            self.freq_map
                .entry(1)
                .or_insert_with(|| LinkedList::new(NodeAdapter::new()))
                .push_back(node_rc);
        }
    }

    /// Removes a node from its current frequency list.
    ///
    /// # Safety
    ///
    /// This function uses an `unsafe` block to remove the node from the
    /// intrusive linked list. It is safe because:
    /// - `node_rc` is guaranteed to be part of the frequency list corresponding to `freq`.
    /// - We have exclusive access to `self` and manage all insertions/removals.
    fn remove_node_from_freq_list(&mut self, node_rc: &Rc<Node>) {
        let freq = node_rc.freq.get();

        if let Some(freq_list) = self.freq_map.get_mut(&freq) {
            unsafe {
                freq_list
                    // the argument *const <A::PointerOps as PointerOps>::Value
                    // here A is NodeAdapter and Value is Node
                    // which means *const Node
                    // So all below syntax are valid
                    //      &*node_rc           // miri error
                    //      node_rc.as_ref()    // miri error
                    //      Rc::as_ptr(&node_rc)
                    .cursor_mut_from_ptr(Rc::as_ptr(node_rc))
                    .remove()
                    .expect("node not found");
            }

            if freq_list.is_empty() {
                self.freq_map.remove(&freq);
                if self.min_freq == freq {
                    self.min_freq += 1;
                }
            }
        }
    }

    fn update(&mut self, node_rc: Rc<Node>) {
        self.remove_node_from_freq_list(&node_rc);

        node_rc.freq.set(node_rc.freq.get() + 1);

        self.freq_map
            .entry(node_rc.freq.get())
            .or_insert_with(|| LinkedList::new(NodeAdapter::new()))
            .push_back(node_rc);
    }
}

/*
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
