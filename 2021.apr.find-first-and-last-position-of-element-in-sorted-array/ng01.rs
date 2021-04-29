impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1]
        }

        let mut start_pointer = nums.len() / 2;
        // println!("{}", start_pointer);
        loop {
            if nums[start_pointer] == target {
                if start_pointer == 0 || nums[start_pointer - 1] != target {
                    break
                }
            }
            if start_pointer == 0 {
                return vec![-1, -1]
            }

            if nums[start_pointer] > target {
                start_pointer = if start_pointer < 2 { 0 } else { start_pointer + start_pointer / 2 };
            } else {
                start_pointer /= 2;
            }

            if start_pointer == nums.len() {
                return vec![-1, -1]
            }
        }
        
        let mut end_pointer = start_pointer + (nums.len() - start_pointer) / 2;

        loop {
            if nums[end_pointer] == target {
                if end_pointer == nums.len() - 1 || nums[end_pointer + 1] != target {
                    break
                }
            }

            if nums[end_pointer] > target {
                end_pointer = if end_pointer < 2 { 0 } else { end_pointer + end_pointer / 2 };
            } else {
                end_pointer /= 2;
            }
        }

        vec![start_pointer as i32, end_pointer as i32]
    }
}