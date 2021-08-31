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
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

fn dig(node: Option<&Rc<RefCell<TreeNode>>>, sums: &mut HashSet<i32>) -> i32 {
    if let Some(node) = node {
        let sum = node.borrow().val + dig(node.borrow().left.as_ref(), sums) + dig(node.borrow().right.as_ref(), sums);
        sums.insert(sum);
        sum
    } else {
        0
    }
}

impl Solution {
    pub fn check_equal_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut subtree_sums: HashSet<i32> = HashSet::new();
        
        let total = dig(root.as_ref(), &mut subtree_sums);
        
        // println!("{:?} {:?}", total, subtree_sums);
        
        // remove total
        subtree_sums.remove(&total);
        
        if total.abs() % 2 == 1 {
            false
        } else {
            if subtree_sums.is_empty() { return true }    
            subtree_sums.get(&(total / 2)).is_some()
        }
    }
}