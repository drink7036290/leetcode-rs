// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

use leetcode_prelude::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;

        let mut dummy_head = Box::new(ListNode::new(0));
        let mut tail = &mut dummy_head;

        let mut carry = 0;
        while (l1.is_some()) || (l2.is_some()) || (carry > 0) {
            let v1 = l1.as_ref().map_or(0, |n| n.val);
            let v2 = l2.as_ref().map_or(0, |n| n.val);

            let mut sum = v1 + v2 + carry;
            if sum > 9 {
                sum -= 10;
                carry = 1;
            } else {
                carry = 0;
            }

            tail.next = Some(Box::new(ListNode::new(sum)));
            tail = tail.next.as_mut().unwrap();

            l1 = l1.and_then(|n| n.next);
            l2 = l2.and_then(|n| n.next);
        }

        dummy_head.next
    }
}
