// https://leetcode.com/submissions/detail/489706070/
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
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

fn make_tree(nodes: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    // println!("nodes: {:?}", nodes);
    let center = nodes.len() / 2;
    // println!("center: {:?}", center);
    if nodes.len() == 1 {
        Some(Rc::new(RefCell::new(TreeNode::new(nodes[center]))))
    } else if nodes.len() == 2 {
        let left = make_tree(&nodes[0..1]);
        Some(Rc::new(RefCell::new(TreeNode { val: nodes[center], left: left, right: None })))
    } else {
        let left = make_tree(&nodes[0..center]);
        let right = make_tree(&nodes[(center + 1)..nodes.len()]);
        Some(Rc::new(RefCell::new(TreeNode { val: nodes[center], left: left, right: right })))
    }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if head == None { return None }
        let mut nodes: Vec<i32> = Vec::new();
        let mut node = head;
        while let Some(current) = node {
            nodes.push(current.val);
            node = current.next;
        }
        
        println!("{:?}", nodes);
        
        make_tree(&nodes[0..])
    }
}