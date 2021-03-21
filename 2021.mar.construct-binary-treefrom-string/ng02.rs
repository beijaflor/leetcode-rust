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


fn get_val(s: &str) -> (i32, &str) {
    let mut closed = false;
    let pos = s.chars().position(|v| v == '(' || v == ')').unwrap();
    (s[..pos].parse().unwrap(), &s[pos..])
}

fn gen_node(s: &str) -> (Option<Rc<RefCell<TreeNode>>>, &str) {
    // println!("s: {:?}", s);
    if let Some(next) = s.chars().next() {        
        if next == ')' {
            // println!("None");
            (None, &s[1..])
        } else {
            // println!("Some");
            let (val,  s) = if s.chars().next().unwrap() == '(' { get_val(&s[1..]) } else { get_val(s) };

            let (left, s) = gen_node(s);
            let (right, s) = if left == None {
                if s.len() == 0 {
                    (None, "")                
                } else {
                    (None, &s[1..])
                }
            } else { gen_node(s) };

            // println!("val {:?}, left {:?}, right {:?}", val, left, right);
            
            (
                Some(Rc::new(RefCell::new(TreeNode {
                    val: val,
                    left: left,
                    right: right,
                }))),
                s,
            )
        }
    } else {
        (None, "")
    }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn str2tree(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let (result, _) = gen_node(&s);
        // println!("RESULT: {:?}\n\n", result);
        result
    }
}
