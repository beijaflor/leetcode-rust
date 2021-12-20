// https://leetcode.com/submissions/detail/592803969/
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            // println!("{} {} {}", lo, hi, mid);
//             if (mid == 0 || nums[mid - 1] < target) && nums[mid] >= target {
//                 return mid
//             }
            if nums[mid] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        
        lo as i32
    }
}