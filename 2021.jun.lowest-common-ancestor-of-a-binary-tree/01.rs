// https://leetcode.com/submissions/detail/515470816/
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
fn dig(root: Option<Rc<RefCell<TreeNode>>>, p: Rc<RefCell<TreeNode>>, q: Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => None,
        Some(root) => {
            // println!("{:?}", root);
            
            // println!("{} {} {}", root.borrow().val, p.borrow().val, q.borrow().val);
            if (root.borrow().val == p.borrow().val || root.borrow().val == q.borrow().val) {
                Some(root)
                // if left != None && right != None {
                // } else {
                // if right != None {
                //     println!("ONLY LEFT!");
                //     right
                // } else {
                //     println!("ONLY RIGHT!");
                //     left
                // }
            } else {
                let left = dig(root.borrow().left.clone(), p.clone(), q.clone());
                let right = dig(root.borrow().right.clone(), p.clone(), q.clone());
                // println!("left: {:?}, right: {:?}", left, right);
                
                if left != None && right != None {
                    // println!("BOTH SOME!");
                    Some(root)
                } else if right != None {
                    // println!("ONLY RIGHT!");
                    right
                } else if left != None {
                    // println!("ONLY LEFT!");
                    left
                } else {
                    None
                }
            }
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // println!("\n\n");
        dig(root, p.unwrap(), q.unwrap())
    }
}