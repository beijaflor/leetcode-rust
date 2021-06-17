impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let mut result = 0;
        let mut subtotal = 0;
        let mut left_pointer = 0;
        let mut right_pointer = 0;
        let len = nums.len();
        while right_pointer < len {
            subtotal += nums[right_pointer];

            while subtotal > right {
                println!("exceed");
                subtotal -= nums[left_pointer];
                left_pointer += 1;
            }

            if subtotal >= left {
                println!("succeed");
                result += 1;
                if right_pointer != left_pointer {
                    result += (right_pointer - left_pointer) as i32;
                }
            }
            right_pointer += 1;
        }
        
        result
    }
}