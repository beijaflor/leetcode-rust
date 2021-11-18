// https://leetcode.com/submissions/detail/582277967/
impl Solution {
    pub fn arrange_coins(mut n: i32) -> i32 {
        // let mut n = (0..31).fold(1i128, |a, _| a * 10) - 1;
        let mut level = 1;
        while n - level >= 0 {
            n -= level;
            level += 1;
        }
        (level - 1) as i32
    }
}