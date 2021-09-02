// https://leetcode.com/submissions/detail/547140074/
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        if nums.len() == 1 || nums[r] >= nums[l] {
            return nums[0]
        }

        while r >= l {
            let mid = l + (r - l) / 2;
            if nums[mid] > nums[mid + 1] {
                return nums[mid + 1]
            }
            
            if nums[mid - 1] > nums[mid] {
                return nums[mid]
            }
            
            if nums[mid] > nums[0] {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        
        -1
    }
}