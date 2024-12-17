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

    link: LinkedListLink,
}

intrusive_adapter!(NodeAdapter = Rc<Node>: Node { link: LinkedListLink });

impl Node {
    pub fn new(key: i32, val: i32) -> Self {
        Node {
            key,
            val: Cell::new(val),
            link: LinkedListLink::default(),
        }
    }
}

pub struct LRUCache {
    map: HashMap<i32, Rc<Node>>,        // key -> node
    freq_list: LinkedList<NodeAdapter>, // list of nodes, ordered by last access time
    capacity: usize,                    // as HashMap's capacity() could be auto-resized
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::with_capacity(capacity as usize),
            freq_list: LinkedList::new(NodeAdapter::new()),
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
                match self.freq_list.pop_front() { Some(node_rc) => {
                    self.map.remove(&node_rc.key);
                } _ => {}}
            }

            let node_rc = Rc::new(Node::new(key, value));
            self.map.insert(key, node_rc.clone());

            self.freq_list.push_back(node_rc);
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
        unsafe {
            self.freq_list
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
    }

    fn update(&mut self, node_rc: Rc<Node>) {
        self.remove_node_from_freq_list(&node_rc);
        self.freq_list.push_back(node_rc);
    }
}

/*
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
