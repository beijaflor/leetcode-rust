// https://leetcode.com/submissions/detail/543919928/
impl Solution {
    pub fn judge_square_sum(mut c: i32) -> bool {
        let mut i = 2;
        while i * i <= c {
            let mut count = 0;
            if c % i == 0 {
                while c % i == 0 {
                    count += 1;
                    c /= i;
                }
                if i % 4 == 3 && count % 2 != 0 {
                    return false
                }
            }
            
            i += 1;
        }

        c % 4 != 3
    }
}