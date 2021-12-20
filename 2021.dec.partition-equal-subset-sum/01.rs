// https://leetcode.com/submissions/detail/600474361/
impl Solution {
    pub fn can_partition(mut nums: Vec<i32>) -> bool {
        if nums.len() == 0 {
            return false
        }
        
        let total: i32 = nums.iter().sum();
        if total % 2 != 0 {
            return false
        }
        let sub = (total / 2) as usize;
        
        let mut dp = vec![false; sub + 1];
        dp[0] = true;
        nums.into_iter().for_each(|current| {
            let current = current as usize;
            (current..=sub).rev().for_each(|index| {
                dp[index] |= dp[index - current];
            });
        });
        
        return dp[sub];
    }
}