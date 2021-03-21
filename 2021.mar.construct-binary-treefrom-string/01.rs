// https://leetcode.com/submissions/detail/470398108/
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
    let pos = s.chars().position(|v| v == '(' || v == ')');
    if pos == None {
        (s.parse().unwrap(), "")
    } else {
        let pos = pos.unwrap();
        (s[..pos].parse().unwrap(), &s[pos..])
    }
}

fn gen_node(s: &str) -> (Option<Rc<RefCell<TreeNode>>>, &str) {
    // println!("s: {:?}", s);
    let (val,  s) = get_val(&s);
    // println!("val: {}, s: {:?}", val, s);

    let next = s.chars().next();
    let (left, s) = if next != None && next.unwrap() == '(' {
        gen_node(&s[1..])
    } else {
        (None, s)
    };
    // println!("left: {:?}, s: {:?}", left, s);

    let next = s.chars().next();
    let (right, s) = if left != None && next != None && next.unwrap() == '(' {
        gen_node(&s[1..])
    } else {
        (None, s)
    };
    // println!("left: {:?}, s: {:?}", right, s);

    let next = s.chars().next();
    let s = if next != None && next.unwrap() == ')' {
        &s[1..]
    } else { s };
    
    (
        Some(Rc::new(RefCell::new(TreeNode {
            val: val,
            left: left,
            right: right,
        }))),
        s,
    )
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn str2tree(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        if s.len() == 0 {
            return None
        }
        gen_node(&s).0
    }
}
