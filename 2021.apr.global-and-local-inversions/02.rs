// https://leetcode.com/submissions/detail/477000836/
impl Solution {
    pub fn is_ideal_permutation(a: Vec<i32>) -> bool {
        for index in 0..a.len() {
            if (a[index] - index as i32).abs() > 1 {
                return false
            }
        }
        true
    }
}