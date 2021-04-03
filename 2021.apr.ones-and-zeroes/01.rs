// https://leetcode.com/submissions/detail/475743398/
impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        // println!("m: {}, n: {}", m, n);
        let m = m as usize;
        let n = n as usize;
        let mut dp: Vec<Vec<i32>> =  vec![vec![0; n + 1]; m + 1];
        for str in strs.into_iter() {
            let mut zeros: usize = 0;
            let mut ones: usize = 0;
            str.chars().for_each(|char| {
                match char {
                    '0' => zeros += 1,
                    '1' => ones += 1,
                    _ => ()// NOP
                }
            });
            // println!("{} {}", zeros, ones);
            for zero in (zeros..m + 1).rev() {
                for one in (ones..n + 1).rev() {
                    // println!("zero : {}, one : {}", zero, one);
                    // println!("zero-: {}, one-: {}", zero - zeros, one - ones);
                    dp[zero][one] = i32::max(1 + dp[zero - zeros][one - ones], dp[zero][one]);
                }
            }
        }
        // println!("{:?}\n\n", dp);
        dp[m][n]
    }
}
