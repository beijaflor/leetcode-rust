// https://leetcode.com/submissions/detail/590227614/
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

fn traverse(inorder: Vec<i32>, postorder: &mut Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    // println!("{:?} {:?}", inorder, postorder);
    if inorder.len() == 0 { return None } 
    if let Some(center) = postorder.pop() {
        // println!("center: {}", center);
        let position = inorder.iter().position(|x| *x == center).unwrap();
        // println!("position: {}", position);
        let left = inorder[0..position].to_vec();
        let right = inorder[position+1..].to_vec();
        // println!("{:?} {:?}", left, right);
        let right_node = traverse(right, postorder);
        let left_node = traverse(left, postorder);
        Some(Rc::new(RefCell::new(
            TreeNode {
                val: center,
                left: left_node,
                right: right_node,
            }
        )))
    } else {
        None
    }
}

impl Solution {
    pub fn build_tree(mut inorder: Vec<i32>, mut postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        traverse(inorder, &mut postorder)
    }
}