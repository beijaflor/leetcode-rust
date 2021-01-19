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

fn create_node(num: i128) -> Option<Box<ListNode>> {
  let val = (num % 10) as i32;
  let ansecter = num / 10;
  // println!("ancester: {}, val: {}", ansecter, val);
  if ansecter == 0 {
      return Some(Box::new(ListNode::new(val)))
  } else {
      let mut node = ListNode::new(val);
      node.next = create_node(ansecter);
      // println!("anc: {:?}", ansecter);
      return Some(Box::new(node))
  }
}

fn get_num(node: &Option<Box<ListNode>>) -> Option<i128> {
  // println!("node: {:?}", node);
  match node {
      None => None,
      Some(node) => {
          if let Some(child) = get_num(&node.next) {
              Some(node.val as i128 + child * 10)
          } else {
              Some(node.val as i128)
          }
      }
  }
}

impl Solution {
  pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      println!("l1: {}", get_num(&l1).unwrap());
      println!("l2: {}", get_num(&l2).unwrap());
      let result = get_num(&l1).unwrap() + get_num(&l2).unwrap();
      // println!("result: {:?}", result);
      create_node(result)
  }
}
