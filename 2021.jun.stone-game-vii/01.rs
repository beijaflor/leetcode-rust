// https://leetcode.com/submissions/detail/506442616/
impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; n];
        let mut prefix_sum: Vec<i32> = Vec::with_capacity(stones.len() + 1);
        prefix_sum.push(0);
        (0..stones.len()).for_each(|index| {
            prefix_sum.push(prefix_sum[index] + stones[index]);
        });
        // println!("{:?}", prefix_sum);

        for len in (2..=n) {
            for start in 0..(n - len + 1) {
                let end = start + len - 1;
                let score_first = prefix_sum[end + 1] - prefix_sum[start + 1];
                let score_last = prefix_sum[end] - prefix_sum[start];
                dp[start][end] = i32::max(
                    score_first - dp[start + 1][end],
                    score_last - dp[start][end - 1],
                )

            }
        }
        
        // println!("{:?}", dp);
        dp[0][n - 1]
    }
}