// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Self {
//     TreeNode {
//       val,
//       left,
//       right
//     }
//   }
// }

fn wrap(node: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
  Some(Rc::new(RefCell::new(node)))
}

fn main() {
  let input = wrap(TreeNode::new(1, None, None));
  assert_eq!(
      vec![1.0],
      average_of_levels(input)
  );
  println!("SUCCESS\n\n");

  let input = wrap(TreeNode::new(3,
      wrap(TreeNode::new(9, None, None)),
      wrap(TreeNode::new(20,
          wrap(TreeNode::new(15, None, None)),
          wrap(TreeNode::new(7, None, None))
      ))
  ));
  assert_eq!(
      vec![3.0, 14.5, 11.0],
      average_of_levels(input)
  );
  println!("SUCCESS\n\n");
}