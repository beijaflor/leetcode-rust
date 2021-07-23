impl Solution {
    pub fn reverse(mut unreversed: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut reversed : Option<Box<ListNode>> = None;

        while let Some(mut node) = unreversed {
            unreversed = node.next;
            node.next = reversed;
            reversed = Some(node);
        }
        reversed
    }

    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut fake_head : ListNode = ListNode::new(0);
        let mut last_ptr : &mut ListNode = &mut fake_head;
        while head.is_some() {
            let mut group_tail_ptr : &mut ListNode = head.as_mut().unwrap();
            let mut i : i32 = 0;
            while i < k - 1 {
                if let Some(ref mut next_ptr) = group_tail_ptr.next {
                    group_tail_ptr = next_ptr;
                    i+=1;
                } else {
                    break;
                }
            }

            if i != k - 1 {
                last_ptr.next = head;
                break;
            }

            let next_head = group_tail_ptr.next.take();
            last_ptr.next = Self::reverse(head);
            while let Some(ref mut next_ptr) = last_ptr.next {
                last_ptr = next_ptr;
            }
            head = next_head;
        }

        fake_head.next
    }
}