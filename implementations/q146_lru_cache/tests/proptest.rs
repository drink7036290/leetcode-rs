use proptest::prelude::*;

use cache_util::*;

fn operation_sequence_strategy() -> impl Strategy<Value = Vec<CacheOperation>> {
    let provider = mock_operations_range_provider_default();

    prop::collection::vec(any::<CacheOperation>(), provider.operations_range())
}

fn test_lru_cache_with_operations(capacity: usize, operations: Vec<CacheOperation>) {
    use q146_lru_cache::intrusive_two_hashmaps::LRUCache as LRUCache_intrusive_two_hashmaps;
    use q146_lru_cache::priority_queue::LRUCache as LRUCache_priority_queue;
    use q146_lru_cache::two_hashmaps::LRUCache as LRUCache_two_hashmaps;
    use q146_lru_cache::vec_hashmap::LRUCache as LRUCache_vec_hashmap;

    let mut cache_priority_queue = LRUCache_priority_queue::new(capacity as i32);
    let mut cache_vec_hashmap = LRUCache_vec_hashmap::new(capacity as i32);
    let mut cache_two_hashmaps = LRUCache_two_hashmaps::new(capacity as i32);
    let mut cache_intrusive_two_hashmaps = LRUCache_intrusive_two_hashmaps::new(capacity as i32);

    for operation in operations {
        match operation {
            CacheOperation::Put { key, value } => {
                cache_priority_queue.put(key, value);
                cache_vec_hashmap.put(key, value);
                cache_two_hashmaps.put(key, value);
                cache_intrusive_two_hashmaps.put(key, value);
            }
            CacheOperation::Get { key } => {
                let result_priority_queue = cache_priority_queue.get(key);
                let result_vec_hashmap = cache_vec_hashmap.get(key);
                let result_two_hashmaps = cache_two_hashmaps.get(key);
                let result_intrusive_two_hashmaps = cache_intrusive_two_hashmaps.get(key);

                // Compare results
                assert_eq!(
                    result_priority_queue, result_vec_hashmap,
                    "priority_queue and vec_hashmap differ on get({})",
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
fn test_lru_cache_implementations() {
    let config = ProptestConfig::with_cases(NUM_PROPTEST_CASES); // Number of test cases to generate
    proptest!(config, |(capacity in capacity_range(), operations in operation_sequence_strategy())| {
        test_lru_cache_with_operations(capacity, operations);
    });
}
