// https://leetcode.com/submissions/detail/557942518/
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let string: Vec<char> = s.chars().collect();
        let target: Vec<char> = t.chars().collect();
        
        let mut dp: Vec<i32> = vec![0; target.len()];
        (0..string.len()).rev().for_each(|p| {
            let mut prev = 1;
            (0..target.len()).rev().for_each(|progress| {
                let old = dp[progress];
                if string[p] == target[progress] {
                    dp[progress] += prev;
                }
                prev = old;
            });
        });
        dp[0]
    }
}