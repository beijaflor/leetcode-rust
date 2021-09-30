// https://leetcode.com/submissions/detail/562372799/
impl Solution {
    pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
        let mut lp = 0;
        let mut rp = 1;
        while lp < nums.len() && rp < nums.len() {
            if nums[lp] % 2 == 0 {
                lp += 2;
                continue
            } 
            if nums[rp] % 2 == 1 {
                rp += 2;
                continue
            }
            nums.swap(lp, rp);
        }
        nums
    }
}