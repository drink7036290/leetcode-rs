use std::ops::RangeInclusive;

use once_cell::sync::Lazy;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

const SEED: u64 = 12345; // Use a fixed seed for reproducibility

const NUM_OPERATIONS: usize = 10_000;

fn key_range() -> RangeInclusive<i32> {
    0..=100_000
}

fn value_range() -> RangeInclusive<i32> {
    0..=1_000_000_000
}

pub static OPERATIONS: Lazy<Vec<(u8, i32, i32)>> = Lazy::new(|| {
    // Generate random data here
    let mut rng = StdRng::seed_from_u64(SEED);

    let mut operations = Vec::with_capacity(NUM_OPERATIONS);

    for _ in 0..NUM_OPERATIONS {
        let op = rng.gen_range(0..2) as u8; // 0 for put, 1 for get
        let key = rng.gen_range(key_range());
        let value = rng.gen_range(value_range()); // Only needed for put operations
        operations.push((op, key, value));
    }

    operations
});

#[macro_export]
macro_rules! bench_lru_cache {
    ($bench_name:ident, $cache_type:ty) => {
        fn $bench_name(c: &mut ::criterion::Criterion) {
            // Define the capacity as a constant or parameter
            const CAPACITY: i32 = 3_000i32;

            c.bench_function(stringify!($bench_name), |b| {
                b.iter(|| {
                    let mut cache: $cache_type =
                        <$cache_type>::new(::criterion::black_box(CAPACITY));
                    for &(op, key, value) in $crate::modules::bench_common::OPERATIONS.iter() {
                        match op {
                            0 => {
                                // Put operation
                                cache.put(
                                    ::criterion::black_box(key),
                                    ::criterion::black_box(value),
                                );
                            }
                            1 => {
                                // Get operation
                                ::criterion::black_box(cache.get(::criterion::black_box(key)));
                            }
                            _ => unreachable!(),
                        }
                    }
                })
            });
        }
    };
}
