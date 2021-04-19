// https://leetcode.com/submissions/detail/482448568/
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
impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if head == None { return None };
        if head.clone().unwrap().next == None { return None };
        let mut count = n;
        let mut resulting_node = head.clone();
        let mut preceding_pointer = head.as_ref();
        let mut decending_pointer = resulting_node.as_mut();
        while let Some(node) = preceding_pointer {
            // println!("{:?}", node);
            preceding_pointer = node.next.as_ref();
            count -= 1;
            if count == -1 {
                decending_pointer = resulting_node.as_mut();
            } else {
                decending_pointer = decending_pointer.unwrap().next.as_mut();
            }
        }
        // println!("count {:?}", count);
        if count < 0 {
            let mut node = decending_pointer.unwrap();
            let next = match node.clone().next { None => None, Some(next_node) => next_node.next };
            // println!("node: {:?}, next: {:?}", node, next);
            node.next = next;
            resulting_node
        } else {
            resulting_node.unwrap().next
        }
    }
}

