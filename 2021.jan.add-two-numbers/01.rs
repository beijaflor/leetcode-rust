// https://leetcode.com/submissions/detail/442384257/
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

fn add(l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>, carry: bool) -> (i32, bool) {
  let val1 = match l1 {
      None => 0,
      Some(node) => node.val,
  };
  let val2 = match l2 {
      None => 0,
      Some(node) => node.val,
  };
  let mut calc = val1 + val2;
  if carry { calc += 1 }
  return (calc % 10, calc > 9)
}

fn adder(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: bool) -> Option<Box<ListNode>> {
  let (result, carry) = add(&l1, &l2, carry);
  let mut node = ListNode::new(result);
  let next1 = match l1 {
      None => None,
      Some(l1) => l1.next
  };
  let next2 = match l2 {
      None => None,
      Some(l2) => l2.next
  };
  if next1 != None || next2 != None {
      node.next = adder(next1, next2, carry);
  } else if carry {
      node.next = Some(Box::new(ListNode::new(1)))
  }
  Some(Box::new(node))
}

impl Solution {
  pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      adder(l1, l2, false)
  }
}
