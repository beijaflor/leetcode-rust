// https://leetcode.com/submissions/detail/471900135/
use std::collections::HashMap;

impl Solution {
    pub fn smallest_common_element(mat: Vec<Vec<i32>>) -> i32 {
        let mut map: HashMap<i32, usize> = HashMap::new();
        let len = mat.len();
        mat.into_iter().for_each(|row| {
            row.into_iter().for_each(|cell| {
                map.entry(cell).and_modify(|c| *c += 1).or_insert(1);
            });
        });
        let result = map.into_iter().find(|(_, count)| {
            *count == len
        });
        match result {
            None => -1,
            Some((val, _)) => val,
        }
    }
}