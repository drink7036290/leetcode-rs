mod modules;

use criterion::{criterion_group, criterion_main};

use q146_lru_cache::impl_v3::LRUCache;

bench_lru_cache!(q146_with_two_hashmaps, LRUCache);

criterion_group!(benches, q146_with_two_hashmaps);
criterion_main!(benches);
