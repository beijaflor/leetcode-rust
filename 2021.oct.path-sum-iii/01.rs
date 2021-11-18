// https://leetcode.com/submissions/detail/572511137/
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
use std::collections::HashMap;

#[derive(Debug)]
struct Solver {
    count: i32,
    k: i32,
    h: HashMap<i32, i32>,
}

impl Solver {
    fn new(sum: i32) -> Self {
        Solver {
            count: 0,
            k: sum,
            h: HashMap::new(), 
        }
    }
    
    fn preorder(&mut self, node: Option<&Rc<RefCell<TreeNode>>>, sum: i32) {
        if let Some(node) = node {
            let current_sum = sum + node.borrow().val;
            // println!("{:?}", self.h);
            // println!("val {}, current_sum {}", node.borrow().val, current_sum);

            if current_sum == self.k {
                self.count += 1;
            }
            
            self.count += self.h.get(&(current_sum - self.k)).unwrap_or(&0);
            
            self.h.insert(current_sum, self.h.get(&current_sum).unwrap_or(&0) + 1);
            
            self.preorder(node.borrow().left.as_ref(), current_sum);
            self.preorder(node.borrow().right.as_ref(), current_sum);
            
            self.h.insert(current_sum, self.h.get(&current_sum).unwrap() - 1);
        }
    }
}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut solver = Solver::new(target_sum);
        // println!("{:?}", solver);
        solver.preorder(root.as_ref(), 0);
        // println!("{:?}", solver);
        solver.count
    }
}