// https://leetcode.com/submissions/detail/519798967/
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = vec![1; nums.len()];
        (0..nums.len()).for_each(|index1| {
            (0..index1).for_each(|index2| {
                if nums[index1] > nums[index2] {
                    dp[index1] = i32::max(dp[index1], dp[index2] + 1);
                }
            });
        });
        dp.into_iter().max().unwrap()
    }
}
/*
class Solution {
    public int lengthOfLIS(int[] nums) {
        int[] dp = new int[nums.length];
        Arrays.fill(dp, 1);
        
        for (int i = 1; i < nums.length; i++) {
            for (int j = 0; j < i; j++) {
                if (nums[i] > nums[j]) {
                    dp[i] = Math.max(dp[i], dp[j] + 1);
                }
            }
        }
        
        int longest = 0;
        for (int c: dp) {
            longest = Math.max(longest, c);
        }
        
        return longest;
    }
}
*/