#[macro_export]
macro_rules! bench_cache {
    ($bench_name:ident, $cache_type:ty) => {
        fn $bench_name(c: &mut ::criterion::Criterion) {
            let capacity = *cache_util::CAPACITY;
            c.bench_function(stringify!($bench_name), |b| {
                b.iter(|| {
                    let mut cache: $cache_type =
                        <$cache_type>::new(::criterion::black_box(capacity as i32));
                    for op in cache_util::OPERATIONS.iter() {
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
                            _ => unreachable!(),
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
    ($crate_name:ident, $impl:ident, $cache_type:ty) => {
        ::paste::paste! {
            use criterion::{criterion_group, criterion_main};

            use $crate_name::$impl::$cache_type as CACHE;

            // Generate a unique benchmark function name
            bench_cache!([<$crate_name _bench_ $impl>], CACHE);

            // Collect the benchmark function into the group
            criterion_group!(benches, [<$crate_name _bench_ $impl>]);
            criterion_main!(benches);
        }
    };
}
