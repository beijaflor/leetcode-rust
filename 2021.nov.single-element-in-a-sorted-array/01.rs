// https://leetcode.com/submissions/detail/589920132/
/*
[1,1,2,3,3,4,4,8,8]
[1,1,2,2,3,3,4,8,8]
[1,2,2,3,3,4,4,8,8]
[1,1,2,2,3,3,4,4,8]
[1,1,2]
[1,2,2]
*/
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        // println!("{:?}", nums);
        let mut lo = 0;
        let mut hi = (nums.len() - 1) / 2;
        while lo < hi {
            let mid = (lo + hi) / 2;
            // println!("{} {} {}", lo, mid, hi);
            if nums[mid * 2] == nums[mid * 2 + 1] {
                lo = mid + 1;
            } else
            if mid == 0 || nums[mid * 2] == nums[mid * 2 - 1] {
                hi = mid;
            } else
            {
                return nums[mid * 2]
            }
        }
        // println!("FINALY: {} {}", lo, hi);
        nums[hi * 2]
    }
}