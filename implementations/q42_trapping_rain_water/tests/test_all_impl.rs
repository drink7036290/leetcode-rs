use rstest::rstest;

use q42_trapping_rain_water::impl_v1::Solution as Solution_v1;
use q42_trapping_rain_water::impl_v2::Solution as Solution_v2;

#[rstest]
#[case(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6)]
#[case(vec![4, 2, 0, 3, 2, 5], 9)]
fn test_all_impl(#[case] height: Vec<i32>, #[case] expected: i32) {
    let result_v1 = Solution_v1::trap(height.clone());
    assert_eq!(expected, result_v1);

    let result_v2 = Solution_v2::trap(height.clone());
    assert_eq!(expected, result_v2);
}
