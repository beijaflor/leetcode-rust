// https://leetcode.com/submissions/detail/481567029/
use std::collections::VecDeque;

impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack: VecDeque<(char, usize)> = VecDeque::new();
        s.chars().for_each(|c| {
            match stack.pop_back() {
                None => stack.push_back((c, 1)),
                Some((last_char, count)) => {
                    if last_char == c {
                        if count + 1 < k as usize {
                            stack.push_back((last_char, count + 1));
                        }
                    } else {
                        stack.push_back((last_char, count));
                        stack.push_back((c, 1));
                    }
                },
            }
        });
        println!("{:?}", stack);
        stack.into_iter().map(|(c, count)| {
            (0..count).map(|_| c).collect::<String>()
        }).collect::<String>()
    }
}