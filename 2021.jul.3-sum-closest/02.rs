// https://leetcode.com/submissions/detail/529155465/
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut diff = i32::MAX;
        let N = nums.len();
        nums.sort();
        for index in 0..N {
            let mut lo = index + 1;
            let mut hi = N - 1;
            while lo < hi {
                let sum = nums[index] + nums[lo] + nums[hi];
                if (target - sum).abs() < diff.abs() {
                    diff = target - sum;
                }
                
                if sum < target {
                    lo += 1;
                } else {
                    hi -= 1;
                }
            }
            if diff == 0 { break }
        }
        
        target - diff
    }
}