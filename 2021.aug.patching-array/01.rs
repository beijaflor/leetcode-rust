// https://leetcode.com/submissions/detail/546070824/
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut patches = 0;
        let mut index = 0;
        let mut miss: i64 = 1;

        while miss <= n as i64 {
            if index < nums.len() && nums[index] as i64 <= miss {
                miss += nums[index] as i64;
                index += 1;
            } else {
                miss += miss;
                patches += 1;
            }
        }
        
        patches
    }
}