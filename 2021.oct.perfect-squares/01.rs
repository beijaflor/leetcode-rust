// https://leetcode.com/submissions/detail/570902895/
fn is_square(n: i32) -> bool {
    let sq = (n as f64).sqrt() as i32;
    sq * sq == n
}

impl Solution {
    pub fn num_squares(mut n: i32) -> i32 {
        while (n % 4 == 0) {
            n /= 4;
        }
        if n % 8 == 7 {
            4
        }
        else if is_square(n) {
            1
        }
        else {
            let mut i = 0;
            while i * i <= n {
                if is_square(n - i * i) {
                    return 2
                }
                i += 1;
            }
            3
        }
    }
}