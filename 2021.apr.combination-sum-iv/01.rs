// https://leetcode.com/submissions/detail/482599967/
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;
        for comb_sum in 1..target + 1 {
            for num in nums.iter() {
                // println!("{}", comb_sum - num);
                if comb_sum - num >= 0 {
                    dp[comb_sum as usize] += dp[(comb_sum - num) as usize];
                }
            }
        }
        dp[target as usize]
    }
}