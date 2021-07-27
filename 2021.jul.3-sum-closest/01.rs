// https://leetcode.com/submissions/detail/529151659/
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut result = -1;
        let mut min = i32::MAX;
        for i1 in 0..nums.len() - 2 {
            for i2 in i1 + 1..nums.len() - 1 {
                for i3 in i2 + 1..nums.len() {
                    let r = nums[i1] + nums[i2] + nums[i3];
                    // println!("{} {}", target, r);
                    let d = if target > r { target - r } else { r - target };
                    if d < min {
                        result = r;
                        min = d;
                    }
                }
            }
        }
        
        result
    }
}