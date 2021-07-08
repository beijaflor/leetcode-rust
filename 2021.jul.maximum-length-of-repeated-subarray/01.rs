// https://leetcode.com/submissions/detail/519326331/
impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut dp: Vec<Vec<i32>> = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
        (0..nums1.len()).rev().for_each(|i1| {
            (0..nums2.len()).rev().for_each(|i2| {
                if nums1[i1] == nums2[i2] {
                    dp[i1][i2] = dp[i1 + 1][i2 + 1] + 1;
                    result = i32::max(result, dp[i1][i2]);
                }
            });
        });
        result
    }
}

/*
class Solution {
    public int findLength(int[] A, int[] B) {
        int ans = 0;
        int[][] memo = new int[A.length + 1][B.length + 1];
        for (int i = A.length - 1; i >= 0; --i) {
            for (int j = B.length - 1; j >= 0; --j) {
                if (A[i] == B[j]) {
                    memo[i][j] = memo[i+1][j+1] + 1;
                    if (ans < memo[i][j]) ans = memo[i][j];
                }
            }
        }
        return ans;
    }
}
*/