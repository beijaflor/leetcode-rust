// https://leetcode.com/submissions/detail/477418346/
impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        if n % 2 == 1 {
            (1..(n / 2 + 1)).map(|i| i * 2).sum()
        } else {
            (0..(n / 2)).map(|i| i * 2 + 1).sum()
        }
    }
}