// https://leetcode.com/submissions/detail/523193776/
impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 3 { return 0 }

        let mut result = 0;
        nums.sort();
        (0..(nums.len() -2)).for_each(|index| {
            let mut k = index + 2;
            if nums[index] == 0 { return }
            ((index + 1)..(nums.len() - 1)).for_each(|j| {
                while k < nums.len() && nums[index] + nums[j] > nums[k] {
                    k += 1;
                }
                result += k - j - 1;
            });
        });
        
        result as i32
    }
}