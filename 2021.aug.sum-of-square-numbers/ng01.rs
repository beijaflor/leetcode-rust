// https://leetcode.com/submissions/detail/543914198/
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut a = 0;
        while a * a <= c {
            let mut b = 0;
            while b * b <= c {
                // println!("{}", a * a + b * b);
                if a * a + b * b == c {
                    return true
                }
                b += 1;
            }
            a += 1;
        }
        false
    }
}