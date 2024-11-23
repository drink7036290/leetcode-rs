use rstest::rstest;

use q1829_maximum_xor_for_each_query::impl_v1::Solution as Solution_v1;

#[rstest]
#[case(vec![0, 1, 2, 2, 5, 7], 3, vec![4, 3, 6, 4, 6, 7])]
#[case(vec![2, 3, 4, 7], 3, vec![5, 2, 6, 5])]
#[case(vec![0, 1, 1, 3], 2, vec![0, 3, 2, 3])]
fn test_all_impl(#[case] nums: Vec<i32>, #[case] maximum_bit: i32, #[case] expected: Vec<i32>) {
    let result_v1 = Solution_v1::get_maximum_xor(nums.clone(), maximum_bit);
    assert_eq!(expected, result_v1);
}
