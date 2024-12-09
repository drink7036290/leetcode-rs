mod modules;

use criterion::{criterion_group, criterion_main};

use q146_lru_cache::impl_intrusive_two_hashmaps::LRUCache;

bench_lru_cache!(q146_with_intrusive_two_hashmaps, LRUCache);

criterion_group!(benches, q146_with_intrusive_two_hashmaps);
criterion_main!(benches);
