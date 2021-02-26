// https://leetcode.com/submissions/detail/461073296/
use std::collections::VecDeque;

impl Solution {
    pub fn validate_stack_sequences(mut pushed: Vec<i32>, mut popped: Vec<i32>) -> bool {
        let mut stack: VecDeque<i32> = VecDeque::new();
        while !(pushed.is_empty() && popped.is_empty()) {
            // println!("stack: {:?}", stack);
            if let Some(last) = stack.iter().next_back() {
                // println!("stack_last: {:?}", last);
                if let Some((popped_first, elements)) = popped.split_first() {
                    // println!("pop_first: {}", popped_first);
                    if last == popped_first {
                        stack.pop_back();
                        popped = elements.to_vec();
                    } else if let Some((pushed_first, elements)) = pushed.split_first() {
                        // println!("pushed_first: {:?}", pushed_first);
                        stack.push_back(*pushed_first);
                        pushed = elements.to_vec();
                    } else {
                        return false
                    }
                } else if let Some((pushed_first, elements)) = pushed.split_first() {
                    // println!("pushed_first: {:?}", pushed_first);
                    stack.push_back(*pushed_first);
                    pushed = elements.to_vec();
                } else {
                    return false
                }
            } else {
                if let Some((first, elements)) = pushed.split_first() {
                    // println!("stack: {:?}", stack);
                    stack.push_back(*first);
                    pushed = elements.to_vec();
                }
            }
            // println!("{:?}, {:?}", pushed, popped);
        }
        stack.is_empty()
    }
}