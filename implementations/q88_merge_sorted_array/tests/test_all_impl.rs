use rstest::rstest;

use q88_merge_sorted_array::impl_v1::Solution as Solution_v1;
use q88_merge_sorted_array::impl_v2::Solution as Solution_v2;
use q88_merge_sorted_array::impl_v3::Solution as Solution_v3;

#[rstest]
#[case(vec![1, 2, 3, 0, 0, 0], vec![2, 5, 6], vec![1, 2, 2, 3, 5, 6])]
#[case(vec![1], vec![], vec![1])]
#[case(vec![0], vec![1], vec![1])]
#[case(vec![1, 2, 3], vec![], vec![1, 2, 3])]
#[case(vec![0, 0, 0], vec![2, 5, 6], vec![2, 5, 6])]
#[case(vec![2, 0], vec![1], vec![1, 2])]
#[case(vec![4, 5, 6, 0, 0, 0], vec![1, 2, 3], vec![1, 2, 3, 4, 5, 6])]
fn test_all_impl(#[case] nums1: Vec<i32>, #[case] nums2: Vec<i32>, #[case] expected: Vec<i32>) {
    let n = nums2.len() as i32;
    let m = nums1.len() as i32 - n;

    let mut result_v1 = nums1.clone();
    Solution_v1::merge(&mut result_v1, m, &nums2, n);
    assert_eq!(expected, result_v1);

    let mut result_v2 = nums1.clone();
    Solution_v2::merge(&mut result_v2, m, &nums2, n);
    assert_eq!(expected, result_v2);

    let mut result_v3 = nums1.clone();
    Solution_v3::merge(&mut result_v3, m as usize, &nums2, n as usize);
    assert_eq!(expected, result_v3);
}
