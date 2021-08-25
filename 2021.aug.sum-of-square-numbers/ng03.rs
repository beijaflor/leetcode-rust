// https://leetcode.com/submissions/detail/543917224/
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut a = 0;
        while a * a <= c {
            let b: f64 = ((c - a * a) as f64).sqrt();
            if b == b.floor() {
                return true
            }
            a += 1;
        }
        false
    }
}