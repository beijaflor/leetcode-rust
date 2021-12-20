// https://leetcode.com/submissions/detail/601172556/
impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut max = 1;
        let mut last = '*';
        let mut current = 0;
        
        s.chars().for_each(|char| {
            if char == last {
                current += 1;
                max = i32::max(max, current);
            } else {
                last = char;
                current = 1;
            }
        });
        
        max
    }
}