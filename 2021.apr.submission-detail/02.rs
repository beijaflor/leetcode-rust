// https://leetcode.com/submissions/detail/486483612/
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        (f64::log10(n as f64) / f64::log10(3.0)) % 1.0 <= 2.0 * std::f64::EPSILON
    }
}