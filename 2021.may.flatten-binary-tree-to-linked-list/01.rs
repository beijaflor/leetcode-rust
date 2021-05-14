// https://leetcode.com/submissions/detail/493146712/
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

fn flat(node: &mut Option<Rc<RefCell<TreeNode>>>) {
    // println!("node: {:?}", node);
    match node {
        None => return,
        Some(node) => {
            if node.borrow().right != None && node.borrow().left != None {
                // println!("both nodes is None");
                let mut right = node.borrow_mut().right.clone();
                flat(&mut right);

                let mut node_tmp = node.borrow_mut().left.clone().unwrap();
                loop {
                    if node_tmp.borrow().right == None {
                        node_tmp.borrow_mut().right = right;
                        break
                    }
                    let new_node = node_tmp.borrow().right.clone().unwrap();
                    node_tmp = new_node;
                }
                
                let mut left = node.borrow_mut().left.clone();
                flat(&mut left);
                node.borrow_mut().right = left;
                node.borrow_mut().left = None;
            } else
            if node.borrow().left != None {
                // println!("left node is not None");
                let mut left = node.borrow_mut().left.clone();
                flat(&mut left);
                node.borrow_mut().right = left;
                node.borrow_mut().left = None;
            } else
            if node.borrow().right != None {
                flat(&mut node.borrow_mut().right);
            }
        }
    }    
}


use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        flat(root); 
        // println!("FINISH\n\n");
    }
}