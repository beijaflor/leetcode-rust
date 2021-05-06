// https://leetcode.com/submissions/detail/489243646/
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0
        }
        let mut steps = 0;
        let mut position = 0;
        while position < nums.len() {
            // println!("position: {}", position);
            steps += 1;

            let mut next_position = 0;
            let mut max = 0;
            for index in 0..nums[position] + 1 {
                // println!("max: {}, next_position: {}", max, next_position);
                
                if position + index as usize == nums.len() - 1 {
                    return steps;
                }
                
                let mut next = index + nums[position + index as usize];
                // println!("next: {}", next);
                if next > max {
                    // println!("index: {}", index);
                    next_position = index;
                    max = index + nums[position + index as usize];
                }
                // println!("next_position: {}", next_position);
            }
            position += next_position as usize;
        }
        
        steps
    }
}