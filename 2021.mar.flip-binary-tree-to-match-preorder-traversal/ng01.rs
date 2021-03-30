/*
[1,null,2,null,3]
[1,3,2]
[1,2,null,3]
[1,3,2]
[1,2]
[2,1]
[1,2,3]
[1,3,2]
[1,2,3]
[1,2,3]
*/
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

fn dfs(node: Option<Rc<RefCell<TreeNode>>>, voyage: &Vec<i32>, flipped: &mut Vec<i32>, mut index: usize) -> usize {
    if let Some(node) = node {
        println!("val: {:?}, index: {}", node.borrow().val, index);
        if index < voyage.len() && node.borrow().val != voyage[index] {
            flipped.clear();
            flipped.push(-1);
            return index
        }
        println!("STEP.1");
        
        index += 1;

        if index < voyage.len() {
            println!("STEP.2");
            let is_flipped = if let Some(left) = node.borrow().left.as_ref() {
                println!("left: {:?}, voyage: {:?}, index: {}", left.borrow().val, voyage[index], index);
                left.borrow().val != voyage[index]
            } else {
                true
            };
            println!("STEP.3");
            if is_flipped {
                flipped.push(node.borrow().val);
                index = dfs(node.borrow().right.clone(), &voyage, flipped, index);
                index = dfs(node.borrow().left.clone(), &voyage, flipped, index);
            } else {
                index = dfs(node.borrow().left.clone(), &voyage, flipped, index);
                index = dfs(node.borrow().right.clone(), &voyage, flipped, index);
            }
        }
    }
    index
}

/*
    public void dfs(TreeNode node) {
        if (node != null) {
            if (node.val != voyage[index++]) {
                flipped.clear();
                flipped.add(-1);
                return;
            }

            if (index < voyage.length && node.left != null &&
                    node.left.val != voyage[index]) {
                flipped.add(node.val);
                dfs(node.right);
                dfs(node.left);
            } else {
                dfs(node.left);
                dfs(node.right);
            }
        }
    }
*/
impl Solution {
    pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
        let mut flipped: Vec<i32> = vec![];
        let mut index: usize = 0;
        
        dfs(root, &voyage, &mut flipped, index);
        println!("{:?} \n\n", flipped);
        if !flipped.is_empty() && flipped[0] == -1 {
            flipped.clear();
            flipped.push(-1);
        }
        flipped
    }
}
/*
class Solution {
    List<Integer> flipped;
    int index;
    int[] voyage;

    public List<Integer> flipMatchVoyage(TreeNode root, int[] voyage) {
        flipped = new ArrayList();
        index = 0;
        this.voyage = voyage;

        dfs(root);
        if (!flipped.isEmpty() && flipped.get(0) == -1) {
            flipped.clear();
            flipped.add(-1);
        }

        return flipped;
    }
}
*/