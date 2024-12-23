use cache_util::*;
use criterion::{criterion_group, criterion_main};

bench_cache!(
    q146_lru_cache_bench_priority_queue,
    q146_lru_cache::priority_queue::LRUCache
);

bench_cache!(
    q146_lru_cache_bench_priority_queue_eviction,
    q146_lru_cache::priority_queue::LRUEvictionCache
);

criterion_group!(
    benches,
    q146_lru_cache_bench_priority_queue,
    q146_lru_cache_bench_priority_queue_eviction,
);
criterion_main!(benches);
