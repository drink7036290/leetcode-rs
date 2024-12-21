use crate::Cache;
use crate::CacheStorage;
use crate::EvictionPolicy;

pub struct GenericCache<P: EvictionPolicy, S: CacheStorage> {
    policy: P,
    storage: S,
    capacity: usize,
}

impl<P: EvictionPolicy, S: CacheStorage> GenericCache<P, S> {
    pub fn new(policy: P, storage: S, capacity: usize) -> Self {
        Self {
            policy,
            storage,
            capacity,
        }
    }

    pub fn is_full(&self) -> bool {
        self.storage.len() == self.capacity
    }
}

impl<P: EvictionPolicy, S: CacheStorage> Cache for GenericCache<P, S> {
    fn put(&mut self, key: i32, value: i32) {
        if self.storage.get(&key).is_none() && self.is_full() {
            self.policy.evict().inspect(|evicted_key| {
                self.storage.remove(evicted_key);
            });
        }

        self.storage.put(key, value);
        self.policy.on_put(key);
    }

    fn get(&mut self, key: &i32) -> Option<i32> {
        self.storage.get(key).inspect(|_| {
            self.policy.on_get(key);
        })
    }
}
