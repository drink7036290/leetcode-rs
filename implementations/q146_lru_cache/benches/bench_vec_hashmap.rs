use cache_util::*;
use criterion::{criterion_group, criterion_main};

/* bench_cache!(
    q146_lru_cache_bench_vec_hashmap,
    q146_lru_cache::vec_hashmap::LRUEvictionCache
); // original design
 */
bench_cache!(
    q146_lru_cache_bench_vec_hashmap_ss,
    q146_lru_cache::vec_hashmap::LRUCache
); // storage separation design

criterion_group!(
    benches,
    //    q146_lru_cache_bench_vec_hashmap,
    q146_lru_cache_bench_vec_hashmap_ss,
);
criterion_main!(benches);
