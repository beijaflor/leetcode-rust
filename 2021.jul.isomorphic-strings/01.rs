// https://leetcode.com/submissions/detail/521220397/
use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map1: HashMap<char, char> = HashMap::new();
        let mut map2: HashMap<char, char> = HashMap::new();
        let chars1 = s.chars().collect::<Vec<char>>();
        let chars2 = t.chars().collect::<Vec<char>>();
        for index in 0..chars1.len() {
            if let Some(entry) = map1.get(&chars1[index]) {
                // println!("{}", entry);
                if *entry != chars2[index] {
                    return false
                }
            } else if let Some(entry) = map2.get(&chars2[index]) {
                // println!("{}", entry);
                if *entry != chars1[index] {
                    return false
                }
            } else {
                map1.insert(chars1[index], chars2[index]);
                map2.insert(chars2[index], chars1[index]);
            };
        }
        true
    }
}