// https://leetcode.com/submissions/detail/598090487/
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
fn dig(node: Option<Box<ListNode>>, val: i32) -> i32 {
    if let Some(node) = node {
        let val = (val << 1) + node.val;
        dig(node.next, val)
    } else {
        val
    }
}

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        dig(head, 0)
    }
}