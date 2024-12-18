use crate::variables_range::*;
use crate::{DefaultOperationsRangeProvider, OperationsRangeProvider};
use once_cell::sync::Lazy;
use proptest::prelude::*;
use rand::distributions::{Distribution, Standard};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

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

pub static OPERATIONS: Lazy<Vec<CacheOperation>> = Lazy::new(|| {
    let mut rng = StdRng::seed_from_u64(SEED);

    DefaultOperationsRangeProvider
        .operations_range()
        .map(|_| rng.r#gen())
        .collect::<Vec<CacheOperation>>()
});
