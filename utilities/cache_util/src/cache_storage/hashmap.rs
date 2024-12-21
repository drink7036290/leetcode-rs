use super::CacheStorage;
use std::collections::HashMap;

pub struct HashMapStorage(HashMap<i32, i32>);

impl HashMapStorage {
    pub fn new(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }
}

impl CacheStorage for HashMapStorage {
    fn put(&mut self, key: i32, value: i32) {
        self.0.insert(key, value);
    }

    fn get(&mut self, key: &i32) -> Option<i32> {
        self.0.get(key).copied()
    }

    fn remove(&mut self, key: &i32) {
        self.0.remove(key);
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
