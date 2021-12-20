// https://leetcode.com/submissions/detail/602915210/
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut dp = vec![vec![0; cols + 1]; rows + 1];
        let mut result = 0;
        (1..=rows).for_each(|y| {
            (1..=cols).for_each(|x| {
                if matrix[y - 1][x - 1] == '1' {
                    
                    dp[y][x] = i32::min(i32::min(dp[y][x - 1], dp[y - 1][x]), dp[y - 1][x - 1]) + 1;
                    result = i32::max(result, dp[y][x]);
                }
            });
        });
        result * result
    }
}