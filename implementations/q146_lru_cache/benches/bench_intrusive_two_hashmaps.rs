use cache_util::*;

use q146_lru_cache::intrusive_two_hashmaps::LRUCache as CACHE;

define_benchmark!(q146_lru_cache, intrusive_two_hashmaps);
