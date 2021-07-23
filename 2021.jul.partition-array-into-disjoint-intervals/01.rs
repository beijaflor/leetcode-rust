// https://leetcode.com/submissions/detail/526477614/
impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut len = 1;
        loop {
            // println!("max {}, len {}", max, len);
            match (len..nums.len()).rev().position(|index| nums[index] < max ) {
                None => break,
                Some(position) => {
                    // println!("p: {}", position);
                    let position = nums.len() - position;
                    (len..position).for_each(|index| max = i32::max(max, nums[index]));
                    len = position;
                }
            }
        }
        
        len as i32
    }
}