// https://leetcode.com/submissions/detail/573776325/
use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::<i32>::new();
        let mut map = HashMap::<i32, i32>::new();
        
        nums2.into_iter().for_each(|num| {
            while !stack.is_empty() {
                if *stack.last().unwrap() < num {
                    map.insert(stack.pop().unwrap(), num);
                } else {
                    break
                }
            }
            stack.push(num);
            // println!("{:?}", stack);
        });

        stack.into_iter().for_each(|num| { map.insert(num, -1); });

        // println!("{:?}", map);
        
        nums1.into_iter().map(|num| {
            *map.get(&num).unwrap()
        }).collect()
    }
}
