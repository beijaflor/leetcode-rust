// https://leetcode.com/submissions/detail/461076701/
use std::collections::VecDeque;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack: VecDeque<i32> = VecDeque::new();
        let mut pushed = VecDeque::from(pushed);
        let mut popped = VecDeque::from(popped);
        while let Some(push) = pushed.pop_front() {
            stack.push_back(push);
            // println!("{:?}, {:?}, {:?}", stack, pushed, popped);
            while !stack.is_empty() && !popped.is_empty() && stack.back().unwrap() == popped.front().unwrap() {
                stack.pop_back();
                popped.pop_front();
            }
        }
        // println!("{:?}, {:?}", stack, popped);
        stack.is_empty() && popped.is_empty()
    }
}