// https://leetcode.com/submissions/detail/510062744/
impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut dp: Vec<Vec<i32>> = vec![vec![0; k + 1]; n + 1];
        let M = 1_000_000_007;
        (1..=n).for_each(|i| {
            (0..=k).for_each(|j| {
                if j == 0 {
                    dp[i][j] = 1;
                } else {
                    let val = (dp[i - 1][j] + M - (if j >= i { dp[i - 1][j - i] } else { 0 })) % M;
                    dp[i][j] = (dp[i][j - 1] + val) % M;
                }
            });
        });
        (dp[n][k] + M - (if k > 0 { dp[n][k - 1] } else { 0 })) % M
    }
}