use crate::Cache;
use crate::CacheStorage;
use crate::{EvictionAsStoragePolicy, EvictionPolicy};

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
pub struct EvictionCache<P: EvictionAsStoragePolicy> {
    policy: P,
    capacity: usize,
}

impl<P: EvictionAsStoragePolicy> EvictionCache<P> {
    pub fn new(policy: P, capacity: usize) -> Self {
        Self { policy, capacity }
    }

    pub fn is_full(&self) -> bool {
        self.policy.len() == self.capacity // hack
    }
}

macro_rules! GenericCacheImpl {
    ($policy:ident, $storage:ident) => {
        fn put(&mut self, key: i32, value: i32) {
            if self.$storage.get(&key).is_none() && self.is_full() {
                self.$policy.evict().inspect(|evicted_key| {
                    self.$storage.remove(evicted_key);
                });
            }
            self.$storage.put(key, value);
            self.$policy.on_put(key);
        }

        fn get(&mut self, key: &i32) -> Option<i32> {
            self.$storage.get(key).inspect(|_| {
                self.$policy.on_get(key);
            })
        }
    };
}

impl<P: EvictionPolicy, S: CacheStorage> Cache for GenericCache<P, S> {
    GenericCacheImpl!(policy, storage);
}

impl<P: EvictionAsStoragePolicy> Cache for EvictionCache<P> {
    GenericCacheImpl!(policy, policy); // hack
}
