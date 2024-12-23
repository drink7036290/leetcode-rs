use cache_util::*;
use criterion::{criterion_group, criterion_main};

bench_cache!(
    q460_lfu_cache_bench_priority_queue,
    q460_lfu_cache::priority_queue::LFUCache
);

bench_cache!(
    q460_lfu_cache_bench_priority_queue_eviction,
    q460_lfu_cache::priority_queue::LFUEvictionCache
);

criterion_group!(
    benches,
    q460_lfu_cache_bench_priority_queue,
    q460_lfu_cache_bench_priority_queue_eviction,
);
criterion_main!(benches);
