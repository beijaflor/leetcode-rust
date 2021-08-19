use std::collections::VecDeque;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<char>>();
        let mut result = 0;
        // let mut dp: Vec<i32> = vec![0; chars.len()];

        if chars[0] == '0' {
            return 0
        }

        if chars.len() == 1 {
            return 1
        }
        
        for index in (0..chars.len()).rev() {
            // println!("{}", index);
            if index < chars.len() - 1 {
                match chars[index] {
                    '0' => {
                        if index == 0 || (chars[index - 1] != '1' && chars[index - 1] != '2') {
                            return 0
                        }
                    },
                    '1' => {
                        if index + 3 > chars.len() || (chars[index + 2] != '0' && chars[index + 1] != '0') {
                            result += 1;
                        }
                    },
                    '2' => {
                        match chars[index + 1] {
                            '0' | '1' | '2' | '3' | '4' | '5' | '6' => {
                                if index + 3 > chars.len() || chars[index + 2] != '0' {
                                    result += 1;
                                }
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            } else {
                match chars[index] {
                    '0' => {},
                    _ => {
                        result += 1;
                    }
                }
            }
        }
        
        result
    }
}