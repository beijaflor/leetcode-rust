// https://leetcode.com/submissions/detail/467288970/
impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if s.is_empty() { return 0 }
        let chars: Vec<char> = s.chars().collect();
        let mut lp = 0;
        let mut rp = s.len() - 1;
        while lp < rp {
            if chars[lp] != chars[rp] {
                return 2
            }
            lp += 1;
            rp -= 1;
        }
        1
    }
}