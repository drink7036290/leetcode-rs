mod modules;

use criterion::{criterion_group, criterion_main};

use q146_lru_cache::impl_v2::LRUCache;

bench_lru_cache!(q146_with_vec_hashmap, LRUCache);

criterion_group!(benches, q146_with_vec_hashmap);
criterion_main!(benches);
