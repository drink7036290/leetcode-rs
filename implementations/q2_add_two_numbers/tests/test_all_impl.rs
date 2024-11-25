use leetcode_prelude::linkedlist;
use leetcode_prelude::ListNode;

use rstest::rstest;

use q2_add_two_numbers::impl_v1::Solution as Solution_v1;
use q2_add_two_numbers::impl_v2::Solution as Solution_v2;

#[rstest]
#[case(linkedlist![2, 4, 3], linkedlist![5, 6, 4], linkedlist![7, 0, 8])]
#[case(linkedlist![0], linkedlist![0], linkedlist![0])]
#[case(linkedlist![9, 9, 9, 9, 9, 9, 9], linkedlist![9, 9, 9, 9], linkedlist![8, 9, 9, 9, 0, 0, 0, 1])]
fn test_all_impl(
    #[case] l1: Option<Box<ListNode>>,
    #[case] l2: Option<Box<ListNode>>,
    #[case] expected: Option<Box<ListNode>>,
) {
    let result_v1 = Solution_v1::add_two_numbers(l1.clone(), l2.clone());
    assert_eq!(expected, result_v1);

    let result_v2 = Solution_v2::add_two_numbers(l1.clone(), l2.clone());
    assert_eq!(expected, result_v2);
}
