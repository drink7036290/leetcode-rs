use cache_util::*;
use criterion::{criterion_group, criterion_main};

bench_cache!(
    q146_lru_cache_bench_vec_hashmap,
    q146_lru_cache::vec_hashmap::LRUCache
);

bench_cache!(
    q146_lru_cache_bench_vec_hashmap_ev,
    q146_lru_cache::vec_hashmap::LRUEvictionCache
);

criterion_group!(
    benches,
    q146_lru_cache_bench_vec_hashmap,
    q146_lru_cache_bench_vec_hashmap_ev,
);
criterion_main!(benches);
