// https://leetcode.com/submissions/detail/539333727/
struct NumArray {
    prefix_sum: Vec<i64>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let mut sum: i64 = 0;
        let prefix_sum: Vec<i64> = nums.into_iter().map(|num| {
            sum += num as i64;
            sum
        }).collect();
        
        println!("{:?}", prefix_sum);
        
        NumArray {
            prefix_sum,
        }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let minus = if left == 0 { 0 } else { self.prefix_sum[left as usize - 1] };
        (self.prefix_sum[right as usize] - minus) as i32
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */