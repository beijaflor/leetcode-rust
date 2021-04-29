// https://leetcode.com/submissions/detail/486484967/
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        f64::fract(f64::log10(n as f64) / f64::log10(3.0)) < 0.0000000001
    }
}