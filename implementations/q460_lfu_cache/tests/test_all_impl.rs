use rstest::rstest;

use q460_lfu_cache::impl_v1::LFUCache as LFUCache_v1;

#[rstest]
#[case(vec!["LFUCache", "put", "put", "get", "put", "get", "get", "put", "get", "get", "get"], 
vec![vec![2], vec![1, 1], vec![2, 2], vec![1], vec![3, 3], vec![2], vec![3], vec![4, 4], vec![1], vec![3], vec![4]],
vec![None, None, None, Some(1), None, Some(-1), Some(3), None, Some(-1), Some(3), Some(4)])]
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

    let mut cache_v1 = LFUCache_v1::new(args_list[0][0]);

    for (i, cmd) in cmds.iter().enumerate().skip(1) {
        let args = &args_list[i];
        let expected = &expected_list[i];

        match *cmd {
            "get" => {
                assert_ne!(*expected, None);

                let key = args[0];

                if let Some(v) = *expected {
                    assert_eq!(cache_v1.get(key), v);
                }
            }
            "put" => {
                assert_eq!(*expected, None);

                let key = args[0];
                let value = args[1];

                cache_v1.put(key, value);
            }
            _ => {
                panic!("Unknown command: {}", cmd);
            }
        }
    }
}
