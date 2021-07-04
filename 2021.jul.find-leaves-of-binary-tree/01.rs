// https://leetcode.com/submissions/detail/517125892/
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
    
    // bool じゃなくて usize 返したら何度も掘らなくていいかも？
    fn dig(&mut self, node: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = node {
            if node.borrow().left == None && node.borrow().right == None {
                let len = self.leaves.len() - 1;
                self.leaves[len].push(node.borrow().val);
                true
            } else {
                if self.dig(node.borrow().left.clone()) {
                    node.borrow_mut().left = None;
                };
                if self.dig(node.borrow().right.clone()) {
                    node.borrow_mut().right = None;
                };
                false
            }
        } else {
            true
        }
    }
    
    fn find(&mut self) {
        loop {
            self.leaves.push(vec![]);
            if self.dig(self.root.clone()) {
                break
            };
        }
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