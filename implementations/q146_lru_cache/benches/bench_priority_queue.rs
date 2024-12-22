use cache_util::*;
use criterion::{criterion_group, criterion_main};

bench_cache!(
    q146_lru_cache_bench_priority_queue,
    q146_lru_cache::priority_queue::LRUEvictionCache
); // original design

bench_cache!(
    q146_lru_cache_bench_priority_queue_ss,
    q146_lru_cache::priority_queue::LRUCache
); // storage separation design

criterion_group!(
    benches,
    q146_lru_cache_bench_priority_queue,
    q146_lru_cache_bench_priority_queue_ss,
);
criterion_main!(benches);
