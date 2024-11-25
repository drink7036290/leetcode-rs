use rstest::rstest;

use q1_two_sum::impl_v1::Solution as Solution_v1;

#[rstest]
#[case(vec![2, 7, 11, 15], 9, vec![0, 1])]
#[case(vec![3, 2, 4], 6, vec![1, 2])]
fn test_all_impl(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: Vec<i32>) {
    let result_v1 = Solution_v1::two_sum(nums.clone(), target);
    assert_eq!(expected, result_v1);
}
