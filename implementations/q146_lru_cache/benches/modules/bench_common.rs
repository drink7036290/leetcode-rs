use once_cell::sync::Lazy;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use cache_util::*;

pub static OPERATIONS: Lazy<Vec<CacheOperation>> = Lazy::new(|| {
    let mut rng = StdRng::seed_from_u64(SEED);

    DefaultOperationsRangeProvider
        .operations_range()
        .map(|_| rng.gen())
        .collect::<Vec<CacheOperation>>()
});

pub static CAPACITY: Lazy<usize> =
    Lazy::new(|| StdRng::seed_from_u64(SEED).gen_range(capacity_range()));

#[macro_export]
macro_rules! bench_lru_cache {
    ($bench_name:ident, $cache_type:ty) => {
        fn $bench_name(c: &mut ::criterion::Criterion) {
            let capacity = *$crate::modules::bench_common::CAPACITY;
            c.bench_function(stringify!($bench_name), |b| {
                b.iter(|| {
                    let mut cache: $cache_type =
                        <$cache_type>::new(::criterion::black_box(capacity as i32));
                    for op in $crate::modules::bench_common::OPERATIONS.iter() {
                        match op {
                            cache_util::CacheOperation::Put { key, value } => {
                                cache.put(
                                    ::criterion::black_box(*key),
                                    ::criterion::black_box(*value),
                                );
                            }
                            cache_util::CacheOperation::Get { key } => {
                                ::criterion::black_box(cache.get(::criterion::black_box(*key)));
                            }
                        }
                    }
                })
            });
        }
    };
}

#[macro_export]
// Define the custom macro using paste for identifier concatenation
macro_rules! define_benchmark {
    ($postfix:ident) => {
        ::paste::paste! {
            use criterion::{criterion_group, criterion_main};

            // Generate the module path based on the postfix
            use q146_lru_cache::[<impl_ $postfix>]::LRUCache;

            // Generate a unique benchmark function name
            bench_lru_cache!([<q146_lru_cache _ bench_ $postfix>], LRUCache);

            // Collect the benchmark function into the group
            criterion_group!(benches, [<q146_lru_cache _ bench_ $postfix>]);
            criterion_main!(benches);
        }
    };
}
