/*
[5,2,6,1]
[5,1,6,2]
[-1]
[-1, -1]
*/
use std::collections::BTreeMap;

// https://stackoverflow.com/questions/48575866/how-to-get-the-lower-bound-and-upper-bound-of-an-element-in-a-btreeset
fn find_after<T>(tree: &BTreeMap<i32, T>, val: i32) -> Option<(&i32, &T)> {
    use std::ops::Bound::*;
    let mut before = tree.range((Unbounded, Excluded(val)));
    // let mut after = tree.range((Excluded(val), Unbounded));

    // (before.next_back(), after.next())
    before.next_back()
}

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        // let mut result: Vec<i32> = vec![0; nums.len()];
        // println!("{:?}", result);
        let mut set: BTreeMap<i32, i32> = BTreeMap::new();
        let mut result = (0..nums.len()).rev().map(|index| {
            println!("{}", nums[index]);
            let last_count = if let Some(count) = find_after(&set, nums[index]) {
                println!("count: {:?}", count);
                count.1 + 1
            } else {
                0
            };
            set.insert(nums[index], last_count);
            last_count
        }).collect::<Vec<i32>>();
        
        println!("set: {:?}", set);
        result.reverse();
        result
    }
}