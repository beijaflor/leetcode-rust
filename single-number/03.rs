use std::collections::HashMap;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result: HashMap<i32, i8> = HashMap::new();
        let mut iterator = nums.into_iter();
        while let Some(num) = iterator.next() {
            let count = result.entry(num).or_insert(0);
            *count += 1;
        }
        let mut ret: i32 = 0;
        for (k, v) in &result {
            if *v == 1 {
                ret = *k
            }
        }
        ret
    }
}
