// https://leetcode.com/submissions/detail/600471584/
impl Solution {
    pub fn can_partition(mut nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return false
        }
        
        nums.sort();

        let mut lp = 0;
        let mut l_sum = nums[lp];
        let mut rp = nums.len() - 1;
        let mut r_sum = nums[rp];
        
        while !(lp + 1 == rp) {
            // println!("{}: {} / {}: {}", lp, l_sum, rp, r_sum);
            if l_sum + nums[lp + 1] < r_sum + nums[rp - 1] {
                lp += 1;
                l_sum += nums[lp];
            } else {
                rp -= 1;
                r_sum += nums[rp];
            }
        }
        
        l_sum == r_sum
    }
}