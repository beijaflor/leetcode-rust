//
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut a = 0;
        while a * a <= c {
            let b = c - a * a;
            let mut i = 1;
            let mut sum = 0;
            while sum < b {
                sum += i;
                i += 2;
            }
            if sum == b {
                return true
            }
            a += 1;
        }
        false
    }
}