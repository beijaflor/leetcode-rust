// https://leetcode.com/submissions/detail/602916920/
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut dp = vec![0; cols + 1];
        let mut result = 0;
        let mut prev = 0;
        (1..=rows).for_each(|y| {
            (1..=cols).for_each(|x| {
                let temp = dp[x];
                if matrix[y - 1][x - 1] == '1' {
                    
                    dp[x] = i32::min(i32::min(dp[x - 1], prev), dp[x]) + 1;
                    result = i32::max(result, dp[x]);
                } else {
                    dp[x] = 0;
                }
                prev = temp;
            });
        });
        result * result
    }
}