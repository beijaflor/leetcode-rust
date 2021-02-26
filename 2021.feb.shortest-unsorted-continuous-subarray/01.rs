// https://leetcode.com/submissions/detail/460875066/
impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        // println!("{:?}", nums);
        let mut cloned = nums.clone();
        cloned.sort();
        // println!("{:?}", cloned);
        if let Some(start) = nums.iter().enumerate().position(|(index, v)| *v != cloned[index]){
            let end = nums.len() - nums.iter().enumerate().rev().position(|(index, v)| *v != cloned[index]).unwrap();
            // println!("start: {:?}, end: {:?}", start, end);
            (end - start) as i32                                
        } else {
            0
        }
    }
}