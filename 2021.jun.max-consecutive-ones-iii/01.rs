// https://leetcode.com/submissions/detail/514903808/
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let mut current = 0;
        let mut window = k;
        let mut rp: usize = 0;
        let mut lp: usize = 0;

        while rp < nums.len() {
            while window == 0 {
                if nums[lp] == 0 {
                    window += 1;
                }
                current -= 1;
                lp += 1;
            }
            
            while rp < nums.len() && window > 0 {
                if nums[rp] == 0 {
                    window -= 1;
                }
                current += 1;
                rp += 1;

                while rp < nums.len() && nums[rp] == 1 {
                    current += 1;
                    rp += 1;
                }
            }

            result = i32::max(result, current);
        }
        
        result
    }
}