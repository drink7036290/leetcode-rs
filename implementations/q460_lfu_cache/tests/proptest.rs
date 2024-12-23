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

fn test_lfu_cache_with_operations(capacity: i32, operations: Vec<CacheOperation>) {
    use q460_lfu_cache::intrusive_two_hashmaps::LFUCache as LFUCache_intrusive_two_hashmaps;
    use q460_lfu_cache::priority_queue::LFUCache as LFUCache_priority_queue;
    use q460_lfu_cache::priority_queue::LFUEvictionCache as LFUEvictionCache_priority_queue;
    use q460_lfu_cache::two_hashmaps::LFUCache as LFUCache_two_hashmaps;
    use q460_lfu_cache::vec_hashmap::LFUCache as LFUCache_vec_hashmap;
    use q460_lfu_cache::vec_hashmap::LFUEvictionCache as LFUEvictionCache_vec_hashmap;

    let mut cache_priority_queue = LFUCache_priority_queue::new(capacity);
    let mut cache_priority_queue_eviction = LFUEvictionCache_priority_queue::new(capacity);
    let mut cache_vec_hashmap = LFUCache_vec_hashmap::new(capacity);
    let mut cache_vec_hashmap_eviction = LFUEvictionCache_vec_hashmap::new(capacity);
    let mut cache_two_hashmaps = LFUCache_two_hashmaps::new(capacity);
    let mut cache_intrusive_two_hashmaps = LFUCache_intrusive_two_hashmaps::new(capacity);

    for operation in operations {
        match operation {
            CacheOperation::Put { key, value } => {
                cache_priority_queue.put(key, value);
                cache_priority_queue_eviction.put(key, value);
                cache_vec_hashmap.put(key, value);
                cache_vec_hashmap_eviction.put(key, value);
                cache_two_hashmaps.put(key, value);
                cache_intrusive_two_hashmaps.put(key, value);
            }
            CacheOperation::Get { key } => {
                let result_priority_queue = cache_priority_queue.get(key);
                let result_priority_queue_eviction = cache_priority_queue_eviction.get(key);
                let result_vec_hashmap = cache_vec_hashmap.get(key);
                let result_vec_hashmap_eviction = cache_vec_hashmap_eviction.get(key);
                let result_two_hashmaps = cache_two_hashmaps.get(key);
                let result_intrusive_two_hashmaps = cache_intrusive_two_hashmaps.get(key);

                // Compare results
                assert_eq!(
                    result_priority_queue, result_priority_queue_eviction,
                    "priority_queue and priority_queue_eviction differ on get({})",
                    key
                );
                assert_eq!(
                    result_priority_queue, result_vec_hashmap,
                    "priority_queue and vec_hashmap differ on get({})",
                    key
                );
                assert_eq!(
                    result_priority_queue, result_vec_hashmap_eviction,
                    "priority_queue and vec_hashmap_eviction differ on get({})",
                    key
                );
                assert_eq!(
                    result_priority_queue, result_two_hashmaps,
                    "priority_queue and two_hashmaps differ on get({})",
                    key
                );
                assert_eq!(
                    result_priority_queue, result_intrusive_two_hashmaps,
                    "priority_queue and intrusive_two_hashmaps differ on get({})",
                    key
                );
            }
        }
    }
}

#[test]
fn test_lfu_cache_implementations() {
    let config = ProptestConfig::with_cases(10); // Number of test cases to generate
    proptest!(config, |(capacity in 1..=10_000i32, operations in operation_sequence_strategy())| {
        test_lfu_cache_with_operations(capacity, operations);
    });
}
