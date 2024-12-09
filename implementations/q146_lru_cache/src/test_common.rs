use proptest::prelude::*;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::ops::RangeInclusive;

pub const SEED: u64 = 12345; // Use a fixed seed for reproducibility
pub const NUM_PROPTEST_CASES: u32 = 10; // Number of proptest cases to generate

// Key: 0 <= key <= 10^5
pub fn key_range() -> RangeInclusive<i32> {
    0..=100_000
}

// Value: 0 <= value <= 10^9
pub fn value_range() -> RangeInclusive<i32> {
    0..=1_000_000_000
}

// Capacity: 1 <= capacity <= 10^4
pub fn capacity_range() -> RangeInclusive<usize> {
    1..=10_000
}

// Number of Operations: At most 2 * 10^5 calls to get and put
// reduce this number to make the tests faster
pub fn operations_range() -> RangeInclusive<usize> {
    if cfg!(miri) {
        1..=200
    } else {
        1..=10_000
    }
}

// Define an enum to represent cache operations
#[derive(Debug, Clone)]
pub enum CacheOperation {
    Put { key: i32, value: i32 },
    Get { key: i32 },
}

impl Distribution<CacheOperation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CacheOperation {
        if rng.gen_bool(0.5) {
            CacheOperation::Put {
                key: rng.gen_range(key_range()),
                value: rng.gen_range(value_range()),
            }
        } else {
            CacheOperation::Get {
                key: rng.gen_range(key_range()),
            }
        }
    }
}

impl Arbitrary for CacheOperation {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        // Define strategies for each variant.
        let put_strategy = (key_range(), value_range())
            .prop_map(|(key, value)| CacheOperation::Put { key, value });

        let get_strategy = key_range().prop_map(|key| CacheOperation::Get { key });

        // Combine the strategies using prop_oneof to randomly choose between them.
        prop_oneof![put_strategy, get_strategy].boxed()
    }
}
