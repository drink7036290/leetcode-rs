mod modules;

use criterion::{criterion_group, criterion_main};

use q146_lru_cache::impl_priority_queue::LRUCache;

bench_lru_cache!(q146_with_priority_queue, LRUCache);

criterion_group!(benches, q146_with_priority_queue);
criterion_main!(benches);
