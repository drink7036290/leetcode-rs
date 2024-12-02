use criterion::{black_box, criterion_group, criterion_main, Criterion};
use once_cell::sync::Lazy;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

const SEED: u64 = 12345; // Use a fixed seed for reproducibility

static OPERATIONS: Lazy<Vec<(u8, i32, i32)>> = Lazy::new(|| {
    // Generate random data here
    let mut rng = StdRng::seed_from_u64(SEED);

    let num_operations = 2000; // Set the number of operations
    let mut operations = Vec::with_capacity(num_operations);

    for _ in 0..num_operations {
        let op = rng.gen_range(0..2) as u8; // 0 for put, 1 for get
        let key = rng.gen_range(0..=100_000i32);
        let value = rng.gen_range(0..=1_000_000_000i32); // Only needed for put operations
        operations.push((op, key, value));
    }

    operations
});

use q146_lru_cache::impl_v1::LRUCache as LRUCache_v1;
use q146_lru_cache::impl_v2::LRUCache as LRUCache_v2;
use q146_lru_cache::impl_v3::LRUCache as LRUCache_v3;
use q146_lru_cache::impl_v4::LRUCache as LRUCache_v4;

macro_rules! bench_lru_cache {
    ($bench_name:ident, $cache_type:ty) => {
        fn $bench_name(c: &mut Criterion) {
            // Define the capacity as a constant or parameter
            const CAPACITY: i32 = 3_000i32;

            c.bench_function(stringify!($bench_name), |b| {
                b.iter(|| {
                    let mut cache: $cache_type = <$cache_type>::new(black_box(CAPACITY));
                    for &(op, key, value) in OPERATIONS.iter() {
                        match op {
                            0 => {
                                // Put operation
                                cache.put(black_box(key), black_box(value));
                            }
                            1 => {
                                // Get operation
                                black_box(cache.get(black_box(key)));
                            }
                            _ => unreachable!(),
                        }
                    }
                })
            });
        }
    };
}

bench_lru_cache!(bench_lru_cache_v1, LRUCache_v1);
bench_lru_cache!(bench_lru_cache_v2, LRUCache_v2);
bench_lru_cache!(bench_lru_cache_v3, LRUCache_v3);
bench_lru_cache!(bench_lru_cache_v4, LRUCache_v4);

criterion_group!(
    benches,
    bench_lru_cache_v1,
    bench_lru_cache_v2,
    bench_lru_cache_v3,
    bench_lru_cache_v4,
);
criterion_main!(benches);
