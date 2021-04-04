// https://leetcode.com/submissions/detail/476197304/
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut depth: Vec<i32> = Vec::new();
        let mut length = 0;
        let mut max = 0;
        depth.push(0);
        s.chars().for_each(|c| {
            // println!("{:?}", depth);
            match c {
                '(' => {
                    depth.push(0);
                },
                ')' => {
                    if let Some(len) = depth.pop() {
                        if let Some(last) = depth.last_mut() {
                            *last += len + 2;
                            max = i32::max(*last, max);
                        } else {
                            depth.push(0);
                        };
                    } else {
                        //
                    }
                },
                _ => ()
            }
        });
        // println!("{}\n\n\n", max);
        max
    }
}