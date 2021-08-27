// https://leetcode.com/submissions/detail/545074395/
use std::collections::HashMap;

impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let mut map: HashMap<String, i32> = HashMap::new();
        strs.into_iter().for_each(|str| {
            let mut chars: Vec<char> = str.chars().collect();
            (0..(1 << chars.len())).for_each(|i| {
                let mut t: Vec<char> = vec![];
                (0..chars.len()).for_each(|j| {
                    if (i >> j) & 1 != 0 {
                        t.push(chars[j]);
                    }
                });
                let t: String = t.into_iter().collect();
                map.entry(t).and_modify(|c| *c += 1).or_insert(1);
            });
        });

        let mut res = -1;
        map.into_iter().for_each(|(key, val)| {
            if val == 1 {
                res = i32::max(res, key.len() as i32);
            }
        });

        res
    }
}