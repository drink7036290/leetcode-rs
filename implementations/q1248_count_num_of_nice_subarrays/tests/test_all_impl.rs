use rstest::rstest;

use q1248_count_num_of_nice_subarrays::impl_v1::Solution as Solution_v1;

#[rstest]
#[case(vec![1, 1, 2, 1, 1], 1, 6)]
#[case(vec![1, 1, 2, 1, 1], 2, 5)]
#[case(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 1, 24)]
#[case(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2, 16)]
#[case(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 3, 0)]
#[case(vec![2, 4, 6], 1, 0)]
#[case(vec![1, 1, 2, 1, 1], 3, 2)]
fn test_all_impl(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
    let result_v1 = Solution_v1::number_of_subarrays(nums.clone(), k);
    assert_eq!(expected, result_v1);
}
