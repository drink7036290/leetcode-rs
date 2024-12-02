use proptest::prelude::*;

// Define an enum to represent cache operations
#[derive(Debug, Clone)]
enum CacheOperation {
    Put { key: i32, value: i32 },
    Get { key: i32 },
}

// Capacity: 1 <= capacity <= 10^4
// Key: 0 <= key <= 10^5
// Value: 0 <= value <= 10^9
// Number of Operations: At most 2 * 10^5 calls to get and put

// Generator for cache operations within specified constraints
fn cache_operation_strategy() -> impl Strategy<Value = CacheOperation> {
    prop_oneof![
        // Generate a 'put' operation with key in [0, 1e5] and value in [0, 1e9]
        (0..=100_000i32, 0..=1_000_000_000i32)
            .prop_map(|(key, value)| CacheOperation::Put { key, value }),
        // Generate a 'get' operation with key in [0, 1e5]
        (0..=100_000i32).prop_map(|key| CacheOperation::Get { key }),
    ]
}

// Generator for operation sequences with length up to 2e5
fn operation_sequence_strategy() -> impl Strategy<Value = Vec<CacheOperation>> {
    prop::collection::vec(cache_operation_strategy(), 1..=200)
}

fn test_lru_cache_with_operations(capacity: i32, operations: Vec<CacheOperation>) {
    use q146_lru_cache::impl_v1::LRUCache as LRUCache_v1;
    use q146_lru_cache::impl_v2::LRUCache as LRUCache_v2;
    use q146_lru_cache::impl_v3::LRUCache as LRUCache_v3;
    use q146_lru_cache::impl_v4::LRUCache as LRUCache_v4;

    let mut cache_v1 = LRUCache_v1::new(capacity);
    let mut cache_v2 = LRUCache_v2::new(capacity);
    let mut cache_v3 = LRUCache_v3::new(capacity);
    let mut cache_v4 = LRUCache_v4::new(capacity);

    for operation in operations {
        match operation {
            CacheOperation::Put { key, value } => {
                cache_v1.put(key, value);
                cache_v2.put(key, value);
                cache_v3.put(key, value);
                cache_v4.put(key, value);
            }
            CacheOperation::Get { key } => {
                let result_v1 = cache_v1.get(key);
                let result_v2 = cache_v2.get(key);
                let result_v3 = cache_v3.get(key);
                let result_v4 = cache_v4.get(key);

                // Compare results
                assert_eq!(result_v1, result_v2, "v1 and v2 differ on get({})", key);
                assert_eq!(result_v1, result_v3, "v1 and v3 differ on get({})", key);
                assert_eq!(result_v1, result_v4, "v1 and v4 differ on get({})", key);
            }
        }
    }
}

#[test]
fn test_lru_cache_implementations() {
    let config = ProptestConfig::with_cases(10); // Number of test cases to generate
    proptest!(config, |(capacity in 1..=3_000i32, operations in operation_sequence_strategy())| {
        test_lru_cache_with_operations(capacity, operations);
    });
}
