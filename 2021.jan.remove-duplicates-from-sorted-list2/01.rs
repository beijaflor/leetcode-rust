// https://leetcode.com/submissions/detail/438924494/
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

fn find_next(mut node: ListNode) -> Option<Box<ListNode>> {
  let val = node.val;
  while let Some(next) = node.next {
      if val != next.val {
          return wrap(*next)
      }
      node = *next;
  }
  None
}

fn wrap(ln: ListNode) -> Option<Box<ListNode>> {
  Some(Box::new(ln))
}

impl Solution {
  pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      let original = head.clone();
      match head {
          None => None,
          Some(mut node) => {
              if let Some(mut next) = node.next {
                  if node.val != next.val {
                      node.next = Solution::delete_duplicates(wrap(*next));
                      return wrap(*node);
                  }
                  return Solution::delete_duplicates(find_next(*next))
              }
              original
          }
      }
  }
}
