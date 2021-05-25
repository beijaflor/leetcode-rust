// https://leetcode.com/submissions/detail/497827438/
impl Solution {
    pub fn to_lower_case(mut s: String) -> String {
        s.chars().map(|c| {
            if c >= 'A' && 'Z' >= c {
                ((c as u8) + 32) as char
            } else {
                c
            }
        }).collect::<String>()
    }
}