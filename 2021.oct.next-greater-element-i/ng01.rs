use std::collections::{BTreeSet, HashMap};

fn find_after(tree: &BTreeSet<i32>, val: i32) -> Option<&i32> {
    use std::ops::Bound::*;
    
    let exact = tree.get(&val);
    if exact == None {
        // let mut before = tree.range((Unbounded, Excluded(val)));
        let mut after = tree.range((Excluded(val), Unbounded));

        // before.next_back()
        after.next()
    } else {
        exact
    }
}

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut max = i32::MIN;
        let mut btree = BTreeSet::<i32>::new();
        let mut map = HashMap::<i32, i32>::new();
        
        nums2.into_iter().rev().for_each(|num| {
            let r = if let Some(after) = find_after(&btree, num) {
                *after
            } else {
                -1
            };
            btree.insert(num);
            map.insert(num, r);
        });
        
        println!("{:?}", map);
        
        nums1.into_iter().map(|num| {
            *map.get(&num).unwrap()
        }).collect()
    }
}