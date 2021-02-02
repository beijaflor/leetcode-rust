// https://leetcode.com/submissions/detail/450056300/
use std::collections::BTreeSet;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut unique: BTreeSet<i32> = BTreeSet::new(); 
        let mut first = std::i32::MAX;
        let mut unique: BTreeSet<i32> = BTreeSet::new(); 
        nums.into_iter().for_each(|mut v| {
            if v % 2 == 1 {
                v *= 2;
            }
            unique.insert(v);
            first = i32::min(first, v);
        });
        // println!("{:?}", unique);

        let mut last = *unique.iter().next_back().unwrap();
        unique.remove(&last);
        let mut prev = last + 1;
        let mut delta = last - first;
        while last % 2 == 0 {
            if last != prev {
                prev = last;
                last /= 2;
                unique.insert(last);
                first = i32::min(last, first);
            }
            last = *unique.iter().next_back().unwrap();
            unique.remove(&last);
            delta = i32::min(delta, last - first)
        }
        // println!("{:?}", unique);
        delta

    }
}