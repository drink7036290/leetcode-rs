use cache_util::*;
use criterion::{criterion_group, criterion_main};

bench_cache!(
    q460_lfu_cache_bench_vec_hashmap,
    q460_lfu_cache::vec_hashmap::LFUCache
);

bench_cache!(
    q460_lfu_cache_bench_vec_hashmap_eviction,
    q460_lfu_cache::vec_hashmap::LFUEvictionCache
);

criterion_group!(
    benches,
    q460_lfu_cache_bench_vec_hashmap,
    q460_lfu_cache_bench_vec_hashmap_eviction,
);
criterion_main!(benches);
