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
        let mut count = n;
        let mut preceding_pointer = head.clone();
        let mut decending_pointer = head.clone();
        while let Some(node) = preceding_pointer {
            println!("{:?}", node);
            preceding_pointer = node.next.clone();
            count -= 1;
            if count == -1 {
                decending_pointer = head.clone();
            } else {
                decending_pointer = decending_pointer.unwrap().next.clone();
            }
        }
        let next = decending_pointer.clone().unwrap().next.clone().unwrap().next.clone();
        let mut node = decending_pointer.unwrap();
        println!("node: {:?}, next: {:?}", node, next);
        node.next = None;
        head
        // Some((*decending_pointer.unwrap()).clone())
    }
}

