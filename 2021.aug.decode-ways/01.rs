// https://leetcode.com/submissions/detail/540432110/
use std::collections::VecDeque;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<char>>();
        // let mut result = 0;
        // let mut dp: Vec<i32> = vec![0; chars.len()];

        if chars[0] == '0' {
            return 0
        }

        let mut two = 1;
        let mut one = 1;
        
        for index in 1..chars.len() {
            let mut current = 0;
            if chars[index] != '0' {
                // println!("0");
                current = one;
            }
            match chars[index - 1] {
                '1' => {
                    // println!("1");
                    current += two;
                },
                '2' => {
                    match chars[index] {
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' => {
                            // println!("2");
                            current += two;
                        }
                        _ => {}
                    }
                },
                _ => {}
            }

            // println!("{} {} {}", one, two, current);
            two = one;
            one = current;
        }
        
        one
    }
}