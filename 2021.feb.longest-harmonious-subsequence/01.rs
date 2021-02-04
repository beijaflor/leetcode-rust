// https://leetcode.com/submissions/detail/451993797/
use std::collections::BTreeMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map: BTreeMap<i32, i32> = BTreeMap::new();
        nums.iter().for_each(|num| {
            map.entry(*num).and_modify(|c| *c += 1).or_insert(1);
        });
        // println!("map: {:?}", map);

        let mut iterator = map.iter();
        if let Some(initial) = iterator.next() {
            let (mut last_num, mut last_count) = initial;
            return iterator.fold(0, |mut acc, (num, count)| {
                if last_num + 1 == *num {
                    acc = i32::max(acc, count + last_count);
                }
                last_num = num;
                last_count = count;
                acc
            })
        }
        0
    }
}