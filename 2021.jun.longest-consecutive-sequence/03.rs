// https://leetcode.com/submissions/detail/503932839/?
use std::collections::BTreeSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 { return 0 }
        let mut heap: BTreeSet<i32> = BTreeSet::new();
        nums.into_iter().for_each(|num| {
            heap.insert(num);
        });
        // println!("{:?}", heap);

        let mut result = 1;
        let mut seq = 1;
        let mut iterator = heap.into_iter();
        let mut last = iterator.next().unwrap();
        while let Some(current) = iterator.next() {
            // println!("{} {}", current, last);
            if current - 1 != last {
                result = i32::max(result, seq);
                seq = 1;
            } else {
                seq += 1;
            }
            last = current;
        }
        result = i32::max(result, seq);
        
        result
    }
}