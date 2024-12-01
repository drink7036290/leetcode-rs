use rstest::rstest;

use q146_lru_cache::impl_v1::LRUCache as LRUCache_v1;
use q146_lru_cache::impl_v2::LRUCache as LRUCache_v2;
use q146_lru_cache::impl_v3::LRUCache as LRUCache_v3;
use q146_lru_cache::impl_v4::LRUCache as LRUCache_v4;

#[rstest]
#[case(vec!["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"], 
vec![vec![2], vec![1, 1], vec![2, 2], vec![1], vec![3, 3], vec![2], vec![4, 4], vec![1], vec![3], vec![4]],
vec![None, None, None, Some(1), None, Some(-1), None, Some(-1), Some(3), Some(4)])]
fn test_all_impl(
    #[case] cmds: Vec<&str>,
    #[case] args_list: Vec<Vec<i32>>,
    #[case] expected_list: Vec<Option<i32>>,
) {
    assert!(!cmds.is_empty());
    assert_eq!(cmds.len(), args_list.len());
    assert_eq!(cmds.len(), expected_list.len());

    // read the first cmd here
    assert_eq!(cmds[0], "LRUCache");
    assert_eq!(expected_list[0], None);

    let mut cache_v1 = LRUCache_v1::new(args_list[0][0]);
    let mut cache_v2 = LRUCache_v2::new(args_list[0][0]);
    let mut cache_v3 = LRUCache_v3::new(args_list[0][0]);
    let mut cache_v4 = LRUCache_v4::new(args_list[0][0]);

    for (i, cmd) in cmds.iter().enumerate().skip(1) {
        let args = &args_list[i];
        let expected = &expected_list[i];

        match *cmd {
            "get" => match *expected {
                Some(v) => {
                    assert_eq!(args.len(), 1);
                    let key = args[0];

                    assert_eq!(cache_v1.get(key), v);
                    assert_eq!(cache_v2.get(key), v);
                    assert_eq!(cache_v3.get(key), v);
                    assert_eq!(cache_v4.get(key), v);
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

                    cache_v1.put(key, value);
                    cache_v2.put(key, value);
                    cache_v3.put(key, value);
                    cache_v4.put(key, value);
                }
            },
            _ => {
                panic!("Unknown command: {}", cmd);
            }
        }
    }
}
