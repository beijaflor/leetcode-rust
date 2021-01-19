// https://leetcode.com/submissions/detail/438505697/
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

fn vec_list_node(node: Option<Box<ListNode>>) -> Vec<ListNode> {
  let mut result: Vec<ListNode> = vec!();
  let mut current = node;
  while let Some(mut n) = current {
      current = n.next;
      n.next = None;
      result.push(*n);
  }
  result
}

fn wrap(ln: ListNode) -> Option<Box<ListNode>> {
  Some(Box::new(ln))
}

impl Solution {
  pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      let vec1 = vec_list_node(l1);
      let vec2 = vec_list_node(l2);
      let mut vec = [vec1, vec2].concat();
      vec.sort_by(|node1, node2| node2.val.cmp(&node1.val));

      let mut iterator = vec.into_iter();
      let first = iterator.next();
      match first {
          None => return None,
          Some(mut rootNode) => {
              while let Some(mut node) = iterator.next() {
                  node.next = wrap(rootNode);
                  rootNode = node;
              }
              return wrap(rootNode)
          }
      }
  }
}
