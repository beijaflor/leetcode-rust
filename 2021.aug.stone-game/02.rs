// https://leetcode.com/submissions/detail/533633505/
impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let N = piles.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; N + 2]; N + 2];
        
        (1..=N).for_each(|size| {
            (0..=(N - size)).for_each(|index| {
                let j = index + size - 1;
                let parity = (j + index + N) % 2;
                if parity == 1 {
                    dp[index + 1][j + 1] = i32::max(piles[index] + dp[index + 2][j + 1], piles[j] + dp[index + 1][j]);
                } else {
                    dp[index + 1][j + 1] = i32::max(-piles[index] + dp[index + 2][j + 1], -piles[j] + dp[index + 1][j]);
                }
            });
        });
        
        dp[1][N] > 0
    }
}