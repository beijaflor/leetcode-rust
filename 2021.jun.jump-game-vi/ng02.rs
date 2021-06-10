use std::collections::BinaryHeap;

impl Solution {
    pub fn max_result(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        (1..nums.len()).for_each(|index| {
            let candidates = if index < k {
                (0..index).map(|j| { nums[j] }).collect::<Vec<i32>>()
            } else {
                ((index - k)..index).map(|j| { nums[j] }).collect::<Vec<i32>>()
            };
            // println!("{:?}", candidates);
            nums[index] += candidates.into_iter().max().unwrap();
        });


        nums[nums.len() - 1]
    }
}
