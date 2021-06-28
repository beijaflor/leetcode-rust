// https://leetcode.com/submissions/detail/514384652/
use std::collections::LinkedList;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = LinkedList::<char>::new();
        s.chars().for_each(|char| {
            // println!("{:?}", stack);
            if let Some(back) = stack.back() {
                if *back == char {
                    // println!("dup");
                    stack.pop_back();
                    return
                }
            }

            // println!("{:?}", char);
            stack.push_back(char);
        });
        
        stack.into_iter().collect::<String>()
    }
}