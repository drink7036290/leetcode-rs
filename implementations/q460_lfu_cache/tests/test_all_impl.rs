use rstest::rstest;

use q460_lfu_cache::intrusive_two_hashmaps::LFUCache as LFUCache_intrusive_two_hashmaps;
use q460_lfu_cache::priority_queue::LFUCache as LFUCache_priority_queue;
use q460_lfu_cache::priority_queue::LFUEvictionCache as LFUEvictionCache_priority_queue;
use q460_lfu_cache::two_hashmaps::LFUCache as LFUCache_two_hashmaps;
use q460_lfu_cache::vec_hashmap::LFUCache as LFUCache_vec_hashmap;
use q460_lfu_cache::vec_hashmap::LFUEvictionCache as LFUEvictionCache_vec_hashmap;

#[rstest]
#[case(vec!["LFUCache", "put", "put", "get", "put", "get", "get", "put", "get", "get", "get"],
vec![vec![2], vec![1, 1], vec![2, 2], vec![1], vec![3, 3], vec![2], vec![3], vec![4, 4], vec![1], vec![3], vec![4]],
vec![None, None, None, Some(1), None, Some(-1), Some(3), None, Some(-1), Some(3), Some(4)])]
#[case(vec!["LFUCache","put","put","put","put","get","get","get","get","put","get","get","get","get","get"],
vec![vec![3],vec![1,1],vec![2,2],vec![3,3],vec![4,4],vec![4],vec![3],vec![2],vec![1],vec![5,5],vec![1],vec![2],vec![3],vec![4],vec![5]],
vec![None,None,None,None,None,Some(4),Some(3),Some(2),Some(-1),None,Some(-1),Some(2),Some(3),Some(-1),Some(5)])]
fn test_all_impl(
    #[case] cmds: Vec<&str>,
    #[case] args_list: Vec<Vec<i32>>,
    #[case] expected_list: Vec<Option<i32>>,
) {
    assert!(!cmds.is_empty());
    assert_eq!(cmds.len(), args_list.len());
    assert_eq!(cmds.len(), expected_list.len());

    // read the first cmd here
    assert_eq!(cmds[0], "LFUCache");
    assert_eq!(expected_list[0], None);

    let mut cache_priority_queue = LFUCache_priority_queue::new(args_list[0][0]);
    let mut cache_priority_queue_eviction = LFUEvictionCache_priority_queue::new(args_list[0][0]);
    let mut cache_vec_hashmap = LFUCache_vec_hashmap::new(args_list[0][0]);
    let mut cache_vec_hashmap_eviction = LFUEvictionCache_vec_hashmap::new(args_list[0][0]);
    let mut cache_two_hashmaps = LFUCache_two_hashmaps::new(args_list[0][0]);
    let mut cache_intrusive_two_hashmaps = LFUCache_intrusive_two_hashmaps::new(args_list[0][0]);

    for (i, cmd) in cmds.iter().enumerate().skip(1) {
        let args = &args_list[i];
        let expected = &expected_list[i];

        match *cmd {
            "get" => match *expected {
                Some(v) => {
                    assert_eq!(args.len(), 1);
                    let key = args[0];

                    assert_eq!(cache_priority_queue.get(key), v);
                    assert_eq!(cache_priority_queue_eviction.get(key), v);
                    assert_eq!(cache_vec_hashmap.get(key), v);
                    assert_eq!(cache_vec_hashmap_eviction.get(key), v);
                    assert_eq!(cache_two_hashmaps.get(key), v);
                    assert_eq!(cache_intrusive_two_hashmaps.get(key), v);
                }
                None => {
                    panic!("expected value should not be None for cmd \"get\"");
                }
            },
            "put" => match *expected {
                Some(_) => {
                    panic!("expected value should be None for cmd \"put\"");
                }
                None => {
                    assert_eq!(args.len(), 2);
                    let key = args[0];
                    let value = args[1];

                    cache_priority_queue.put(key, value);
                    cache_priority_queue_eviction.put(key, value);
                    cache_vec_hashmap.put(key, value);
                    cache_vec_hashmap_eviction.put(key, value);
                    cache_two_hashmaps.put(key, value);
                    cache_intrusive_two_hashmaps.put(key, value);
                }
            },
            _ => {
                panic!("Unknown command: {}", cmd);
            }
        }
    }
}
