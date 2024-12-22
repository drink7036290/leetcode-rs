use cache_util::HashMapStorage;
use cache_util::{Cache /* , EvictionCache */, GenericCache};
use cache_util::{EvictionPolicyVHM, KeyAwareHeapNode, LRUHeapNode};

pub struct LRUCache {
    cache: GenericCache<EvictionPolicyVHM<KeyAwareHeapNode<LRUHeapNode>>, HashMapStorage>,
}
/*
pub struct LRUEvictionCache {
    cache: EvictionCache<EvictionPolicyVHM<KeyAwareHeapNode<LRUHeapNode>>>,
}
 */
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            cache: GenericCache::new(
                EvictionPolicyVHM::<KeyAwareHeapNode<LRUHeapNode>>::new(),
                HashMapStorage::new(capacity as usize),
                capacity as usize,
            ),
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.cache.put(key, value);
    }

    pub fn get(&mut self, key: i32) -> i32 {
        self.cache.get(&key).unwrap_or(-1)
    }
}
/*
impl LRUEvictionCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            cache: EvictionCache::new(
                EvictionPolicyVHM::<KeyAwareHeapNode<LRUHeapNode>>::new(),
                capacity as usize,
            ),
        }
    }
    pub fn put(&mut self, key: i32, value: i32) {
        self.cache.put(key, value);
    }

    pub fn get(&mut self, key: i32) -> i32 {
        self.cache.get(&key).unwrap_or(-1)
    }
}
 */
/*
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
