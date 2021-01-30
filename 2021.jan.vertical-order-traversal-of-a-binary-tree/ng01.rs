// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;

fn dig(current: Rc<RefCell<TreeNode>>, store: &mut Vec<Vec<i32>>, mut index: usize) -> usize {
    // println!("current: {:?}", current);
    // println!("store: {:?}", store);
    // println!("index: {:?}", index);
    // println!("");

    if let Some(left) = &current.borrow().left {
        if index == 0 {
            store.insert(0, vec![]);
            index += 1;
        }
        store[index - 1].push(left.borrow().val);
        index = dig(Rc::clone(left), store, index - 1);
        index += 1;
    };

    if let Some(right) = &current.borrow().right {
        if index == store.len() - 1 {
            store.push(vec![]);
        }
        store[index + 1].push(right.borrow().val);
        index = dig(Rc::clone(right), store, index + 1);

        index -= 1;
    };
    
    index
}

    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let current = root.unwrap();
        let mut result: Vec<Vec<i32>> = vec![vec![current.borrow().val]];
        dig(current, &mut result, 0);
        println!("{:?}", result);
        result
    }

fn main() {

    let test_data = Some(Rc::new(RefCell::new({TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new({TreeNode {
            val: 9,
            left: None,
            right: None,
        }}))),
        right: Some(Rc::new(RefCell::new({TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new({TreeNode {
                val: 15,
                left: None,
                right: None,
            }}))),
            right: Some(Rc::new(RefCell::new({TreeNode {
                val: 7,
                left: None,
                right: None,
            }}))),
        }}))),
    }})));
    assert_eq!(
        vec![
            vec![9],
            vec![3,15],
            vec![20],
            vec![7],
        ],
        vertical_traversal(test_data)
    );
    println!("SUCCESS\n\n");

    let test_data = Some(Rc::new(RefCell::new({TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new({TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new({TreeNode {
                val: 4,
                left: None,
                right: None,
            }}))),
            right: Some(Rc::new(RefCell::new({TreeNode {
                val: 5,
                left: None,
                right: None,
            }}))),
        }}))),
        right: Some(Rc::new(RefCell::new({TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new({TreeNode {
                val: 6,
                left: None,
                right: None,
            }}))),
            right: Some(Rc::new(RefCell::new({TreeNode {
                val: 7,
                left: None,
                right: None,
            }}))),
        }}))),
    }})));
    assert_eq!(
        vec![
            vec![4],
            vec![2],
            vec![1,5,6],
            vec![3],
            vec![7],
        ],
        vertical_traversal(test_data)
    );
    println!("SUCCESS\n\n");

    let test_data = Some(Rc::new(RefCell::new({TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new({TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new({TreeNode {
                val: 4,
                left: None,
                right: None,
            }}))),
            right: Some(Rc::new(RefCell::new({TreeNode {
                val: 6,
                left: None,
                right: None,
            }}))),
        }}))),
        right: Some(Rc::new(RefCell::new({TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new({TreeNode {
                val: 5,
                left: None,
                right: None,
            }}))),
            right: Some(Rc::new(RefCell::new({TreeNode {
                val: 7,
                left: None,
                right: None,
            }}))),
        }}))),
    }})));
    assert_eq!(
        vec![
            vec![4],
            vec![2],
            vec![1,5,6],
            vec![3],
            vec![7],
        ],
        vertical_traversal(test_data)
    );
    println!("SUCCESS\n\n");
}
