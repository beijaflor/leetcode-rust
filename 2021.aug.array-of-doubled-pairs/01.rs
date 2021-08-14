// https://leetcode.com/submissions/detail/
use std::collections::HashMap;

impl Solution {
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        let mut count: HashMap<i32, i32> = HashMap::new();
        
        arr.iter().for_each(|num| { count.entry(*num).and_modify(|c| *c += 1).or_insert(1); });
        
        // println!("{:?}", count);
        
        let mut sorted: Vec<i32> = arr.clone();
        sorted.sort_by(|lhv, rhv| lhv.abs().cmp(&rhv.abs()));
        
        // println!("{:?}", sorted);

        for num in sorted.into_iter() {
            // println!("{}, {:?}", num, count.get(&num));
            if *count.get(&num).unwrap_or(&0) == 0 { continue }
            if *count.get(&(num * 2)).unwrap_or(&0) <= 0 { return false }
            
            count.entry(num).and_modify(|c| *c -= 1);
            count.entry(num * 2).and_modify(|c| *c -= 1);
            
            // println!("{:?}", count);
        }
        
        true
    }
}