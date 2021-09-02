// https://leetcode.com/submissions/detail/548338274/
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

fn wrap(node: Option<TreeNode>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = node {
        Some(Rc::new(RefCell::new(node)))
    } else {
        None
    }
}

fn make_trees(v: &Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if v.len() == 0 {
        vec![None]
    } else
    if v.len() == 1 {
        vec![wrap(Some(TreeNode::new(v[0])))]
    } else {
        let mut result = vec![];
        (0..v.len()).for_each(|index| {
            let mid = v[index];
            let left = make_trees(&v[0..index].to_vec());
            let right = make_trees(&v[index + 1..v.len()].to_vec());
            left.iter().for_each(|lnode| {
                right.iter().for_each(|rnode| {
                    result.push(wrap(Some(TreeNode { val: mid, left: lnode.clone(), right: rnode.clone() })))
                });
            });
        });        
        result
    }
}

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let v: Vec<i32> = (1..=n).collect();
        make_trees(&v)
    }
}