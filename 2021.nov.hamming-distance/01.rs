// https://leetcode.com/submissions/detail/589398385/
impl Solution {
    pub fn hamming_distance(mut x: i32, mut y: i32) -> i32 {
        let mut result = 0;
        while x > 0 || y > 0 {
            if x % 2 != y % 2 {
                result += 1;
            }
            x = x / 2;
            y = y / 2;
        }
        
        result
    }
}