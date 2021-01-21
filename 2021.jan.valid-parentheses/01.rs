// https://leetcode.com/submissions/detail/445708383/
use std::collections::VecDeque;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let p_open: Vec<char> = "({[".chars().collect();
        let p_close: Vec<char> = ")}]".chars().collect();

        let chars: Vec<char> = s.chars().collect();
        let mut queue: VecDeque<char> = VecDeque::new();
        for char in chars.iter() {
            // println!("char: {:?}", char);
            if p_open.iter().find(|v| v == &char) != None {
                queue.push_back(*char);
            } else if let Some(position) = p_close.iter().position(|v| v == char) {
                // println!("position: {:?}, char: {:?}", position, char);
                match queue.pop_back() {
                    None => return false,
                    Some(stack) => {
                        if p_open[position] != stack {
                           return false
                        }
                    }
                }
            }
        }
        if queue.len() != 0 { false } else { true }
    }
}