// https://leetcode.com/submissions/detail/552053418/
use std::collections::HashSet;

impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut banned: HashSet<i32> = HashSet::new();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; n];
        
        mines.into_iter().for_each(|mine| {
            banned.insert(mine[0] * n as i32 + mine[1]);
        });
        
        let mut result = 0;

        (0..n).for_each(|r| {
            let mut count = 0;
            (0..n).for_each(|c| {
                count = if banned.contains(&((r * n + c) as i32)) {
                    0
                } else {
                    count + 1
                };
                dp[r][c] = count;
            });

            let mut count = 0;
            (0..n).rev().for_each(|c| {
                count = if banned.contains(&((r * n + c) as i32)) {
                    0
                } else {
                    count + 1
                };
                dp[r][c] = i32::min(dp[r][c], count);
            });
        });

        // println!("{:?}", dp);
        
        (0..n).for_each(|c| {
            let mut count = 0;
            (0..n).for_each(|r| {
                count = if banned.contains(&((r * n + c) as i32)) {
                    0
                } else {
                    count + 1
                };
                dp[r][c] = i32::min(dp[r][c], count);
            });

            let mut count = 0;
            (0..n).rev().for_each(|r| {
                count = if banned.contains(&((r * n + c) as i32)) {
                    0
                } else {
                    count + 1
                };
                dp[r][c] = i32::min(dp[r][c], count);
                result = i32::max(result, dp[r][c]);
            });
        });

        // println!("{:?}", dp);

        result
    }
}