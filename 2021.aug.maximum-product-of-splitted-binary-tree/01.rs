// https://leetcode.com/submissions/detail/540873533/
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

struct Calculator {
    sums: Vec<i64>,
    total_sum: i64,
}

impl Calculator {
    fn new(node: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut calc = Calculator {
            sums: vec![],
            total_sum: 0,
        };
        
        let total = calc.tree_sum(node.as_ref());
        calc.total_sum = total;
        calc
    }
    
    fn get(&self) -> i32 {
        let mut best = 0;
        self.sums.iter().for_each(|sum| {
            // println!("{}", sum);
            best = i64::max(best, sum * ( self.total_sum - sum));
        });

        (best % 1_000_000_007) as i32
    }
    
    fn tree_sum(&mut self, node: Option<&Rc<RefCell<TreeNode>>>) -> i64 {
        if let Some(node) = node {
            let left_sum = self.tree_sum(node.borrow().left.as_ref());
            let right_sum = self.tree_sum(node.borrow().right.as_ref());
            let total = left_sum + right_sum + node.borrow().val as i64;
            self.sums.push(total);
            total
        } else {
            0
        }
    }
}


impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let calc = Calculator::new(root);
        // println!("{:?}, {:?}", calc.total_sum, calc.sums);
        calc.get()
    }
}