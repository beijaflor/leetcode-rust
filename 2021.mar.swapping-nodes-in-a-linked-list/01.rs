// https://leetcode.com/submissions/detail/467852924/
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
    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut node_vec: Vec<i32> = vec![];
        let mut pointer = head;
        while let Some(node) = pointer {
            node_vec.push(node.val);
            pointer = node.next;
        }
        let mut tail_index = node_vec.len() - k as usize;
        node_vec.swap(k as usize - 1, tail_index);
        println!("{:?}", node_vec);
        node_vec.into_iter().rev().fold(None, |node, current| {
           Some(Box::new(ListNode { val: current, next: node})) 
        })
    }
}