use cache_util::HashMapStorage;
use cache_util::{Cache, EvictionCache, GenericCache};
use cache_util::{
    EvictionPolicyVHM, KeyAwareHeapNode, LFUHeapNode, LRUHeapNode, ValueAwareHeapNode,
};

pub struct LFUCache {
    cache:
        GenericCache<EvictionPolicyVHM<KeyAwareHeapNode<LFUHeapNode<LRUHeapNode>>>, HashMapStorage>,
}

pub struct LFUEvictionCache {
    cache: EvictionCache<
        EvictionPolicyVHM<ValueAwareHeapNode<KeyAwareHeapNode<LFUHeapNode<LRUHeapNode>>>>,
    >,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            cache: GenericCache::new(
                EvictionPolicyVHM::<KeyAwareHeapNode<LFUHeapNode<LRUHeapNode>>>::new(),
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

impl LFUEvictionCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            cache:
                EvictionCache::new(
                    EvictionPolicyVHM::<
                        ValueAwareHeapNode<KeyAwareHeapNode<LFUHeapNode<LRUHeapNode>>>,
                    >::new(),
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
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
