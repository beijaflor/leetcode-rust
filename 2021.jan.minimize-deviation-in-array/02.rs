// https://leetcode.com/submissions/detail/450061310/
use std::collections::BTreeSet;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut evens: BTreeSet<i32> = BTreeSet::new(); 
        let mut min = std::i32::MAX;
        nums.iter().for_each(|v| {
            let num = *v * ( *v % 2 + 1);
            evens.insert(num);
            min = i32::min(min, num);
        });


        let mut diviation = std::i32::MAX;

        while !evens.is_empty() {
            let current = *evens.iter().next_back().unwrap();
            evens.remove(&current);
            diviation = i32::min(diviation, current - min);
            if current % 2 == 0 {
                evens.insert(current / 2);
                min = i32::min(min, current / 2);
            } else {
                break;
            }
        }

        diviation
    }
}