// https://leetcode.com/submissions/detail/517216995/
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let M = 1_000_000_007;

        let mut dp: Vec<Vec<i64>> = vec![vec![0; 5]; n as usize];
        (0..5).for_each(|i| dp[0][i] = 1);
        
        (1..n as usize).for_each(|i| {
            dp[i][0] = dp[i - 1][1];
            dp[i][1] = (dp[i - 1][0] + dp[i - 1][2]) % M;
            dp[i][2] = (dp[i - 1][0] + dp[i - 1][1] + dp[i - 1][3] + dp[i - 1][4]) % M;
            dp[i][3] = (dp[i - 1][2] + dp[i - 1][4]) % M;
            dp[i][4] = dp[i - 1][0];
        });

        let len = dp.len() - 1;
        dp[len].iter().fold(0, |acc, x| (acc + x) % M) as i32
    }
}