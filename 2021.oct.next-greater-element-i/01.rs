// https://leetcode.com/submissions/detail/573771682/
use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut max = i32::MIN;
        let mut map = HashMap::<i32, i32>::new();
        
        nums2.clone().into_iter().enumerate().rev().for_each(|(index, num)| {
            if max < num {
                map.insert(num, -1);
                max = num;
            } else {
                let greater = nums2.iter().skip(index).find(|greater| *greater > &num).unwrap();
                map.insert(num, *greater);
            }
        });
        
        // println!("{:?}", map);
        
        nums1.into_iter().map(|num| {
            *map.get(&num).unwrap()
        }).collect()
    }
}