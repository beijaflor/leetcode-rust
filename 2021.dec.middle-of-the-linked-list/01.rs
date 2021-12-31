// https://leetcode.com/submissions/detail/608288506/
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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast_cur = &head;
        let mut slow_cur = &head;
        let mut is_odd = false;
        while let Some(node) = fast_cur {
            if is_odd {
                slow_cur = &slow_cur.as_ref().unwrap().next;
            }
            fast_cur = &node.next;
            is_odd = !is_odd;
        }
        slow_cur.clone()
    }
}