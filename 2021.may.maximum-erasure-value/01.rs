// https://leetcode.com/submissions/detail/499460253/
use std::collections::HashMap;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut current = 0;
        let mut pointer: usize = 0;
        let mut map: HashMap<i32, usize> = HashMap::new();
        nums.iter().enumerate().for_each(|(index, num)| {
            let find = map.get_mut(&num);
            current += num;
            match find {
                None => {
                    result = i32::max(result, current);
                },
                Some(index) => {
                    // println!("index: {:?}", index);
                    let index = *index;
                    while pointer <= index {
                        current -= nums[pointer];
                        map.remove(&nums.get(pointer).unwrap());
                        pointer += 1;
                    }
                    // println!("{}", current);
                },
            }
            map.insert(*num, index);
            // println!("index: {}, num: {}", index, num);
        });
        
        result
    }
}