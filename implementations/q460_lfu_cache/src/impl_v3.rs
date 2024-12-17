use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    key: i32,
    val: i32,
    freq: usize,

    prev: Option<Weak<RefCell<Node>>>, // avoid cyclic reference
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(key: i32, val: i32) -> Self {
        Node {
            key,
            val,
            freq: 1,
            prev: None,
            next: None,
        }
    }
}

#[derive(Debug)]
struct FreqList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl FreqList {
    pub fn new() -> Self {
        FreqList {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, node: &Rc<RefCell<Node>>) {
        match self.tail.as_ref() {
            Some(tail) => {
                tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(Rc::downgrade(&tail.clone()));
            }
            None => {
                self.head = Some(node.clone());
            }
        }

        self.tail = Some(node.clone());
    }

    pub fn pop_front(&mut self) -> Option<Rc<RefCell<Node>>> {
        let head = self.head.clone();
        let head_next = head.as_ref().and_then(|node| node.borrow().next.clone());

        self.head = head_next.clone();
        match head_next.as_ref() {
            Some(node) => {
                node.borrow_mut().prev = None;
            }
            None => {
                self.tail = None;
            }
        }

        head
    }

    pub fn remove(&mut self, node_rc: Rc<RefCell<Node>>) {
        // replace clone() with upgrade() here to avoid one time clone()
        let prev_node = node_rc
            .borrow()
            .prev
            .as_ref()
            .and_then(|weak| weak.upgrade());
        let next_node = node_rc.borrow().next.clone();

        match &prev_node {
            Some(prev_rc) => {
                prev_rc.borrow_mut().next = next_node.clone();
            }
            _ => {
                self.head = next_node.clone();
            }
        }

        match &next_node {
            Some(next_rc) => {
                next_rc.borrow_mut().prev = prev_node.as_ref().map(Rc::downgrade);
            }
            _ => {
                self.tail = prev_node.clone();
            }
        }

        node_rc.borrow_mut().prev = None;
        node_rc.borrow_mut().next = None;
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

pub struct LFUCache {
    map: HashMap<i32, Rc<RefCell<Node>>>, // key -> node
    freq_map: HashMap<usize, FreqList>,   // freq -> list of nodes, ordered by last access time
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
        match self.map.get(&key).cloned() {
            Some(node_rc) => {
                let val = node_rc.borrow().val;
                self.update(node_rc);
                val
            }
            _ => -1,
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        // trick: cloned() to avoid multiple mutable self, also cloned Rc is cheap
        match self.map.get(&key).cloned() {
            Some(node_rc) => {
                node_rc.borrow_mut().val = value;
                self.update(node_rc);
            }
            _ => {
                if self.map.len() == self.capacity {
                    if let Some(freq_list) = self.freq_map.get_mut(&self.min_freq) { if let Some(node_rc) = freq_list.pop_front() {
                        self.map.remove(&node_rc.borrow().key);

                        if freq_list.is_empty() {
                            self.freq_map.remove(&self.min_freq);
                        }
                    } }
                }

                let node_rc = Rc::new(RefCell::new(Node::new(key, value)));
                self.map.insert(key, node_rc.clone());
                self.min_freq = 1;

                let freq_list = self.freq_map.entry(1).or_insert_with(FreqList::new);
                freq_list.push_back(&node_rc);
            }
        }
    }

    fn update(&mut self, node_rc: Rc<RefCell<Node>>) {
        let mut freq = node_rc.borrow().freq;

        if let Some(freq_list) = self.freq_map.get_mut(&freq) {
            freq_list.remove(node_rc.clone());

            if freq_list.is_empty() {
                self.freq_map.remove(&freq);
                if self.min_freq == freq {
                    self.min_freq += 1;
                }
            }
        }

        freq += 1;

        let freq_list = self.freq_map.entry(freq).or_insert_with(FreqList::new);

        node_rc.borrow_mut().freq = freq;
        freq_list.push_back(&node_rc);
    }
}

/*
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
