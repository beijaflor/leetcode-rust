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

struct LeavesFinder {
    root: Option<Rc<RefCell<TreeNode>>>,
    leaves: Vec<Vec<i32>>,
}

impl LeavesFinder {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        LeavesFinder {
            root: root,
            leaves: vec![],
        }
    }
    
    fn dig(&mut self, node: Option<Rc<RefCell<TreeNode>>>, depth: usize) {
        if let Some(node) = node {
             if node.borrow().left == None && node.borrow().right == None {
                 while self.leaves.len() <= depth {
                     self.leaves.push(vec![]);
                 }
                 
                 self.leaves[depth].push(node.borrow().val);
            }
            self.dig(node.borrow().left.clone(), depth + 1);
            self.dig(node.borrow().right.clone(), depth + 1);
        }
    }
    
    fn find(&mut self) {
        self.dig(self.root.clone(), 0);
    }
    
    fn get(&self) -> Vec<Vec<i32>> {
        self.leaves.clone()
    } 
}

impl Solution {
    pub fn find_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut finder = LeavesFinder::new(root);
        finder.find();
        finder.get()
    }
}