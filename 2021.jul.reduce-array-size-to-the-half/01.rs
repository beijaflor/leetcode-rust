// https://leetcode.com/submissions/detail/518231274/
use std::collections::BTreeMap;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut map: BTreeMap<i32, i32> = BTreeMap::new();
        let mut result = 0;
        let half = (arr.len() / 2) as i32;
        arr.into_iter().for_each(|num| {
            map.entry(num).and_modify(|c| *c += 1).or_insert(1);
        });
        let mut tmp = 0;
        let mut counts = map.into_iter().map(|(_, count)| count).collect::<Vec<i32>>();
        counts.sort();
        counts.reverse();
        for count in counts.into_iter() {
            tmp += count;
            result += 1;
            if tmp >= half {
                break
            }
        }
        
        // println!("result: {}\n\n", result);
        result
    }
}