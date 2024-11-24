use rstest::rstest;

use q1574_shortest_subarray_to_be_removed_to_make_array_sorted::impl_v1::Solution as Solution_v1;
use q1574_shortest_subarray_to_be_removed_to_make_array_sorted::impl_v2::Solution as Solution_v2;

#[rstest]
#[case(vec![0, 16, 3, 13, 14, 11, 1, 24, 20, 20, 18, 15, 20], 10)]
#[case(vec![2, 7, 4, 6, 11, 17, 4, 1, 5, 13, 10, 12], 8)]
#[case(vec![6, 3, 10, 11, 15, 20, 13, 3, 18, 12], 8)]
#[case(vec![1, 2, 3, 10, 0, 7, 8, 9], 2)]
#[case(vec![5, 4, 3, 4, 5, 2, 1], 6)]
#[case(vec![4, 5, 6, 1, 2, 3], 3)]
#[case(vec![1, 2, 3, 10, 4, 2, 3, 5], 3)]
#[case(vec![5, 4, 3, 2, 1], 4)]
#[case(vec![1, 2, 3], 0)]
#[case(vec![1, 1, 1], 0)]
#[case(vec![1], 0)]
#[case(vec![1, 2], 0)]
#[case(vec![2, 1], 1)]
fn test_all_impl(#[case] arr: Vec<i32>, #[case] expected: i32) {
    let result_v1 = Solution_v1::find_length_of_shortest_subarray(arr.clone());
    assert_eq!(expected, result_v1);

    let result_v2 = Solution_v2::find_length_of_shortest_subarray(arr.clone());
    assert_eq!(expected, result_v2);
}
