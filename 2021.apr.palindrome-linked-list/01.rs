// https://leetcode.com/submissions/detail/475076390/
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
fn node_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut head = head;
    while let Some(h) = head {
        result.push(h.val);
        head = h.next;
    }
    result
}


impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let vec: Vec<i32> = node_to_vec(head);
        let mut cloned = vec.clone();
        cloned.reverse();
        vec == cloned
    }
}