// https://leetcode.com/submissions/detail/475077297/
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
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut vec: Vec<i32> = Vec::new();
        let mut head = head;
        while let Some(h) = head {
            vec.push(h.val);
            head = h.next;
        }

        let mut cloned = vec.to_owned();
        cloned.reverse();
        vec == cloned
    }
}