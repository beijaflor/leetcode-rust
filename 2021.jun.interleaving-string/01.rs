// https://leetcode.com/submissions/detail/501851867/
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s3.len() != s1.len() + s2.len() { return false }
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        
        let mut dp: Vec<Vec<bool>> = vec![vec![false; s2.len() + 1]; s1.len() + 1];
        for i in 0..(s1.len() + 1) {
            for j in 0..(s2.len() + 1) {
                let index = i + j - 1;
                dp[i][j] = if i == 0 && j == 0 {
                    true
                } else if i == 0 {
                    dp[i][j - 1] && s2[j - 1] == s3[index]
                } else if j == 0 {
                    dp[i - 1][j] && s1[i - 1] == s3[index]
                } else {
                    (dp[i - 1][j] && s1[i - 1] == s3[index]) || (dp[i][j - 1] && s2[j - 1] == s3[index])
                }
            }
        }
        // println!("{:?}", dp);
        dp[s1.len()][s2.len()]
    }
}