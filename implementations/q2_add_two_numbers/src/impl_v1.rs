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

        let mut carry = 0;
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut dummy_head;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let l1_val = l1.as_ref().map_or(0, |node| node.val);
            let l2_val = l2.as_ref().map_or(0, |node| node.val);
            let mut sum = carry + l1_val + l2_val;

            carry = 0;
            if sum >= 10 {
                carry = 1;
                sum -= 10;
            }

            let tail_node = tail.as_mut().unwrap();
            tail_node.next = Some(Box::new(ListNode::new(sum)));
            tail = &mut tail_node.next;

            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        dummy_head.unwrap().next
    }
}
